use byteorder::ByteOrder;
use crate::{ByteBuffer, PutResult, TakeResult};

pub trait BufferPut {
    fn put<B: ByteOrder>(self, buffer: &mut ByteBuffer<B>) -> PutResult<B>;
}

pub trait BufferTake {
    fn take<B: ByteOrder>(buffer: &mut ByteBuffer<B>) -> TakeResult<Self>
    where
        Self: Sized;
}

impl BufferPut for u8 {
    fn put<B: ByteOrder>(self, buffer: &mut ByteBuffer<B>) -> PutResult<B> {
        buffer.put_u8(self)
    }
}

impl BufferTake for u8 {
    fn take<B: ByteOrder>(buffer: &mut ByteBuffer<B>) -> TakeResult<u8> {
        buffer.take_u8()
    }
}

impl BufferPut for i8 {
    fn put<B: ByteOrder>(self, buffer: &mut ByteBuffer<B>) -> PutResult<B> {
        buffer.put_i8(self)
    }
}

impl BufferTake for i8 {
    fn take<B: ByteOrder>(buffer: &mut ByteBuffer<B>) -> TakeResult<i8> {
        buffer.take_i8()
    }
}

impl BufferPut for u16 {
    fn put<B: ByteOrder>(self, buffer: &mut ByteBuffer<B>) -> PutResult<B> {
        buffer.put_u16(self)
    }
}

impl BufferTake for u16 {
    fn take<B: ByteOrder>(buffer: &mut ByteBuffer<B>) -> TakeResult<u16> {
        buffer.take_u16()
    }
}

impl BufferPut for i16 {
    fn put<B: ByteOrder>(self, buffer: &mut ByteBuffer<B>) -> PutResult<B> {
        buffer.put_i16(self)
    }
}

impl BufferTake for i16 {
    fn take<B: ByteOrder>(buffer: &mut ByteBuffer<B>) -> TakeResult<i16> {
        buffer.take_i16()
    }
}

impl BufferPut for u32 {
    fn put<B: ByteOrder>(self, buffer: &mut ByteBuffer<B>) -> PutResult<B> {
        buffer.put_u32(self)
    }
}

impl BufferTake for u32 {
    fn take<B: ByteOrder>(buffer: &mut ByteBuffer<B>) -> TakeResult<u32> {
        buffer.take_u32()
    }
}

impl BufferPut for i32 {
    fn put<B: ByteOrder>(self, buffer: &mut ByteBuffer<B>) -> PutResult<B> {
        buffer.put_i32(self)
    }
}

impl BufferTake for i32 {
    fn take<B: ByteOrder>(buffer: &mut ByteBuffer<B>) -> TakeResult<i32> {
        buffer.take_i32()
    }
}

impl BufferPut for u64 {
    fn put<B: ByteOrder>(self, buffer: &mut ByteBuffer<B>) -> PutResult<B> {
        buffer.put_u64(self)
    }
}

impl BufferTake for u64 {
    fn take<B: ByteOrder>(buffer: &mut ByteBuffer<B>) -> TakeResult<u64> {
        buffer.take_u64()
    }
}

impl BufferPut for i64 {
    fn put<B: ByteOrder>(self, buffer: &mut ByteBuffer<B>) -> PutResult<B> {
        buffer.put_i64(self)
    }
}

impl BufferTake for i64 {
    fn take<B: ByteOrder>(buffer: &mut ByteBuffer<B>) -> TakeResult<i64> {
        buffer.take_i64()
    }
}

impl BufferPut for u128 {
    fn put<B: ByteOrder>(self, buffer: &mut ByteBuffer<B>) -> PutResult<B> {
        buffer.put_u128(self)
    }
}

impl BufferTake for u128 {
    fn take<B: ByteOrder>(buffer: &mut ByteBuffer<B>) -> TakeResult<u128> {
        buffer.take_u128()
    }
}

impl BufferPut for i128 {
    fn put<B: ByteOrder>(self, buffer: &mut ByteBuffer<B>) -> PutResult<B> {
        buffer.put_i128(self)
    }
}

impl BufferTake for i128 {
    fn take<B: ByteOrder>(buffer: &mut ByteBuffer<B>) -> TakeResult<i128> {
        buffer.take_i128()
    }
}

impl BufferPut for &[u8] {
    fn put<B: ByteOrder>(self, buffer: &mut ByteBuffer<B>) -> PutResult<B> {
        buffer.put_slice(self)
    }
}
