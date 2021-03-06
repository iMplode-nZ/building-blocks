use crate::{
    ArrayN, BincodeCompression, BytesCompression, Compressed, Compression, IntoRawBytes,
    MaybeCompressed,
};

use building_blocks_core::prelude::*;

/// A compression algorithm for arrays that avoid the overhead of serialization but ignores endianness and therefore isn't
/// portable.
#[derive(Clone, Copy, Debug)]
pub struct FastArrayCompression<N, T, B> {
    pub bytes_compression: B,
    marker: std::marker::PhantomData<(N, T)>,
}

impl<N, T, B> FastArrayCompression<N, T, B> {
    pub fn new(bytes_compression: B) -> Self {
        Self {
            bytes_compression,
            marker: Default::default(),
        }
    }
}

/// A compressed `ArrayN` that decompresses quickly but only on the same platform where it was compressed.
#[derive(Clone)]
pub struct FastCompressedArray<N, T, B> {
    compressed_bytes: Vec<u8>,
    extent: ExtentN<N>,
    marker: std::marker::PhantomData<(T, B)>,
}

impl<N, T, B> FastCompressedArray<N, T, B> {
    pub fn compressed_bytes(&self) -> &[u8] {
        &self.compressed_bytes
    }

    pub fn extent(&self) -> &ExtentN<N> {
        &self.extent
    }
}

impl<N, T, B> Compression for FastArrayCompression<N, T, B>
where
    B: BytesCompression,
    T: 'static + Copy,
    PointN<N>: IntegerPoint<N>,
{
    type Data = ArrayN<N, T>;
    type CompressedData = FastCompressedArray<N, T, B>;

    // Compress the map in-memory using some `B: BytesCompression`.
    //
    // WARNING: For performance, this reinterprets the inner vector as a byte slice without accounting for endianness. This is
    // not compatible across platforms.
    fn compress(&self, data: &Self::Data) -> Compressed<Self> {
        let mut compressed_bytes = Vec::new();
        self.bytes_compression
            .compress_bytes(data.into_raw_bytes(), &mut compressed_bytes);

        Compressed::new(FastCompressedArray {
            extent: data.extent,
            compressed_bytes,
            marker: Default::default(),
        })
    }

    fn decompress(compressed: &Self::CompressedData) -> Self::Data {
        let num_points = compressed.extent.num_points();

        // Allocate the vector with element type T so the alignment is correct.
        let mut decompressed_values: Vec<T> = Vec::with_capacity(num_points);
        unsafe { decompressed_values.set_len(num_points) };
        let mut decompressed_bytes = unsafe {
            std::slice::from_raw_parts_mut(
                decompressed_values.as_mut_ptr() as *mut u8,
                num_points * core::mem::size_of::<T>(),
            )
        };
        B::decompress_bytes(&compressed.compressed_bytes, &mut decompressed_bytes);

        ArrayN::new(compressed.extent, decompressed_values)
    }
}

pub type BincodeArrayCompression<N, T, B> = BincodeCompression<ArrayN<N, T>, B>;
pub type BincodeCompressedArray<N, T, B> = Compressed<BincodeArrayCompression<N, T, B>>;

macro_rules! define_conditional_aliases {
    ($backend:ident) => {
        use crate::$backend;

        pub type MaybeCompressedArrayN<N, T, B = $backend> =
            MaybeCompressed<ArrayN<N, T>, Compressed<FastArrayCompression<N, T, B>>>;
        pub type MaybeCompressedArray2<T, B = $backend> = MaybeCompressedArrayN<[i32; 2], T, B>;
        pub type MaybeCompressedArray3<T, B = $backend> = MaybeCompressedArrayN<[i32; 3], T, B>;

        pub type MaybeCompressedArrayRefN<'a, N, T, B = $backend> =
            MaybeCompressed<&'a ArrayN<N, T>, &'a Compressed<FastArrayCompression<N, T, B>>>;
        pub type MaybeCompressedArrayRef2<'a, T, B = $backend> =
            MaybeCompressedArrayRefN<'a, [i32; 2], T, B>;
        pub type MaybeCompressedArrayRef3<'a, T, B = $backend> =
            MaybeCompressedArrayRefN<'a, [i32; 3], T, B>;
    };
}

// LZ4 and Snappy are not mutually exclusive, but if we only use one, then we want to have these aliases refer to the choice we
// made.
#[cfg(all(feature = "lz4", not(feature = "snap")))]
define_conditional_aliases!(Lz4);
#[cfg(all(not(feature = "lz4"), feature = "snap"))]
define_conditional_aliases!(Snappy);
