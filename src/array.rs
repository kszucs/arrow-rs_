use alloc::raw_vec::RawVec;
use std::convert::From;

// struct ArrayData {
//     dtype: DataType,
//     length: i64,
//     null_count: i64,
//     offset: i64,
//     buffers: Vec<Buffer>,
//     child_data: Vec<ArrayData>
// }

// struct Array {
//     data: ArrayData,
//     null_bitmap_data: *const u8
// }

// struct ArrayBuilder {
//     dtype: DataType,
//     pool: MemoryPool,

//     // When null_bitmap are first appended to the builder, the null bitmap is allocated
//     null_bitmap: Buffer,
//     null_count: i64,
//     null_bitmap_data: *mut u8,

//     // Array length, so far. Also, the index of the next element to be added
//     length: i64,
//     capacity: i64,

//     // Child value array builders. These are owned by this class
//     children: Vec<ArrayBuilder>
// }

// struct Buffer {
//     is_mutable: bool,
//     data: *const u8,
//     mutable_data: *mut u8,
//     size: i64,
//     capacity: i64,
//     parent: Box<Buffer>
// }

// // MutableBuffer : Buffer
// // PoolBuffer : MutableBuffer

// struct BufferBuilder {
//     buffer: Buffer,
//     pool: Mempool,
//     data: *const u8,
//     capacity: i64,
//     size: i64
// }

// struct Mempool {
//     lock: Mutex,
//     bytes_allocated: Atomic<i64>,
//     max_memory: Atomic<i64>
// }


// TODO design it based on purely rust semantics, not c++


//ArrayMut::from(DataType::Int8)

// struct Buffer<T, A: Alloc>(
//     raw: RawVec<T, A>,
//     len: i32
// );


// TODO: custom allocator in mempool
// the following is using the default heap without 64 byte aligned allocation


// struct Numeric<T>; //enum

// static int8: Numeric<i8> = ();
// static int16: Numeric<i16> = ();
// static int32: Numeric<i32> = ();
// static int64: Numeric<i64> = ();

//use types::DataType;



// struct Array<T = DataType> {
//     len: usize,
//     dtype: T,
//     buffers: Vec<Buffer<T>>,
//     children: Option<Vec<Array<T>>>
// }


// impl Array for ArrayData<Primitive> {

//}


// struct List<T: Array> {
//     len: usize,
//     nulls: BitMap,
//     offsets: Buffer<u32>,
//     values: Vec
// }

// struct Struct {
//     len: usize,
//     nulls: BitMap,
//     children: Vec<ArrayData>
// }


// struct Struct<A: Array> {
//     len: usize,
//     nulls: BitMap,
//     children: Vec<A>
// }


// struct DenseUnion<A: Array> {
//     len: usize,
//     nulls: BitMap,
//     types: Buffer<i8>,
//     offsets: Buffer<i32>,
//     children: Vec<A>
// }


// struct SparseUnion<A: Array> {
//     len: usize,
//     nulls: BitMap,
//     types: Buffer<i8>,
//     children: Vec<A>
// }


use dtypes::{DataType, PrimitiveType, ListType, List};


pub type Buffer<T> = RawVec<T>;
pub type BitMap = Buffer<bool>;


pub struct PrimitiveData<T: PrimitiveType>{
    values: Buffer<T::Item>
}


pub struct ListData<T: DataType> {
    offsets: Buffer<u32>,
    values: Array<T>
}


impl<T: PrimitiveType> PrimitiveData<T> {

    pub fn new() -> Self {
        PrimitiveData { values: Buffer::new() }
    }

}


impl<T: DataType> ListData<T> {

    pub fn new(dtype: T) -> Self {
        ListData {
            offsets: Buffer::new(),
            values: Array::new(dtype)
        }
    }
}



struct Array<T: DataType> {
    // atomic stuff etc.
    len: usize,
    dtype: T,
    nulls: BitMap,
    data: T::Data
}


impl<T> Array<T> where T: DataType + Copy {

    fn new(dtype: T) -> Self {
        let data = dtype.empty();
        Array {
            len: 0,
            dtype: dtype,
            nulls: BitMap::new(),
            data: data
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn dtype(&self) -> T { 
        self.dtype
    }

//     /// Remove an item from the end of the vector and return it, or None if empty.
//     #[inline]
//     pub fn pop(&mut self) -> Option<A::Item> {
//         if self.len == 0 {
//             return None
//         }
//         let last_index = self.len - 1;
//         if (last_index as isize) < 0 {
//             panic!("overflow")
//         }
//         unsafe {
//             let end_ptr = self.as_ptr().offset(last_index as isize);
//             let value = ptr::read(end_ptr);
//             self.set_len(last_index);
//             Some(value)
//         }
// }

}


impl<T> Array<T> where T: PrimitiveType {

    pub fn push(&mut self, value: T::Item) {    
        // let cap = self.capacity();
        // if self.len == cap {
        //     self.grow(cmp::max(cap * 2, 1))
        // }
        // unsafe {
        //     let end = self.as_mut_ptr().offset(self.len as isize);
        //     ptr::write(end, value);
        //     let len = self.len;
        //     self.set_len(len + 1)
        // }
    }

}
// impl<T> Array<List<T>> where T: DataType + Copy {


//     fn pina() -> Option<Self> {
//         None
//     }

// }


// impl to_dtype static method for struct with arrow procedural macro


#[cfg(test)]
mod tests {
    use super::*;
    use dtypes::*;

    #[test]
    fn test_from_dtype() {
        Array::new(Float32);
        Array::new(Float64);

        Array::new(Int8);
        Array::new(Int16);
        Array::new(Int32);
        Array::new(Int64);
        Array::new(UInt8);
        Array::new(UInt16);
        Array::new(UInt32);
        Array::new(UInt64);

        Array::new(List(Int64));
        Array::new(List(Float64));
    }

    #[test]
    fn test_simple() {
        let a = Array::new(Int64);

        assert_eq!(a.len(), 0);
        assert_eq!(a.dtype(), Int64);
        
    }

    // #[test]
    // fn test_from_vec() {
    //     let v = vec![1, 2, 3, 4];

    //     let a = Array::from(v);
    // }
}


// struct ArrayMut<T> {
//     data: Vec<T>
// }


// impl<T> ArrayMut<T> {

// }


// unsigned and signed integer should be represented as followineg
// struct UInt(bytes); // or bits
// struct Int(bytes);


// const uint64 = UInt(8);
// const uint32 = UInt(4);
// const uint16 = UInt(2);
// const uint16 = UInt(1);


// enum Primitive {
//     Int8,
//     Int16,
//     Int32,
//     Int64
//     UInt8,
//     UInt16,
//     UInt32,
//     UInt64
// }



//vector.freeze() -> array()
