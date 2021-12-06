use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;
use std::ops::RangeInclusive;
use byteorder::ByteOrder;
use crate::{BufferPut, BufferTake};

type Result<T> = std::result::Result<T, ByteBufferError>;

pub type PutResult<'a, B> = std::result::Result<&'a mut ByteBuffer<B>, BufferOverflowError>;
pub type TakeResult<T> = std::result::Result<T, BufferUnderflowError>;

#[derive(Debug)]
pub enum ByteBufferError {
    IndexOutOfBounds(usize, RangeInclusive<usize>),
}

impl fmt::Display for ByteBufferError {
    fn fmt(&self, _: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

#[derive(Debug)]
pub struct BufferOverflowError {
    requested: usize,
    remaining: usize,
}

#[derive(Debug)]
pub struct BufferUnderflowError {
    requested: usize,
    remaining: usize,
}

impl Error for BufferOverflowError {}
impl Error for BufferUnderflowError {}

impl Display for BufferOverflowError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            BufferOverflowError { requested, remaining } => {
                write!(formatter, "Buffer overflow: requested {} bytes from buffer with {} bytes remaining", requested, remaining)
            }
        }
    }
}

impl Display for BufferUnderflowError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "UNDERFLOW")
    }
}

pub struct ByteBuffer<B: ByteOrder> {
    buffer: Vec<u8>,
    position: usize,
    limit: usize,
    byte_order: PhantomData<B>,
}

impl<B: ByteOrder> ByteBuffer<B> {
    pub fn allocate(capacity: usize) -> Self {
        ByteBuffer::wrap(vec![0u8; capacity])
    }

    pub fn wrap(vec: Vec<u8>) -> Self {

        ByteBuffer {
            limit: vec.len(),
            buffer: vec,
            position: 0,
            byte_order: PhantomData
        }
    }

    pub fn copy_from(slice: &[u8]) -> Self {
        ByteBuffer::wrap(slice.to_vec())
    }

    pub fn get_capacity(&self) -> usize {
        self.buffer.capacity()
    }

    pub fn get_remaining(&self) -> usize {
        self.limit - self.position
    }

    pub fn get_position(&self) -> usize {
        self.position
    }

    pub fn get_limit(&self) -> usize {
        self.limit
    }

    pub fn set_position(&mut self, position: usize) -> Result<()> {
        if position > self.limit {
            Err(ByteBufferError::IndexOutOfBounds(position, 0..=self.limit))
        } else {
            self.position = position;
            Ok(())
        }
    }

    pub fn set_limit(&mut self, limit: usize) -> Result<()> {
        if limit > self.get_capacity() {
            Err(ByteBufferError::IndexOutOfBounds(limit, 0..=self.get_capacity()))
        } else {
            self.limit = limit;
            if self.position > limit {
                self.position = limit;
            }
            Ok(())
        }
    }

    pub fn has_remaining(&self) -> bool {
        self.position < self.limit
    }

    pub fn flip(&mut self) -> &mut Self {
        self.limit = self.position;
        self.position = 0;
        self
    }

    pub fn clear(&mut self) -> &mut Self {
        self.position = 0;
        self.limit = self.get_capacity();
        self
    }

    pub fn compact(&mut self) -> &mut Self {
        self.buffer.copy_within(self.position..self.limit, 0);
        self.position = self.get_remaining();
        self.limit = self.get_capacity();
        self
    }

    pub fn put(&mut self, value: impl BufferPut) -> PutResult<B> {
        value.put(self)
    }

    pub fn put_u8(&mut self, value: u8) -> PutResult<B> {
        self._put(1, |buffer, value| buffer[0] = value, value)
    }

    pub fn put_i8(&mut self, value: i8) -> PutResult<B> {
        self.put_u8(value as u8)
    }

    pub fn put_u16(&mut self, value: u16) -> PutResult<B> {
        self._put(2, B::write_u16, value)
    }

    pub fn put_i16(&mut self, value: i16) -> PutResult<B> {
        self._put(2, B::write_i16, value)
    }

    pub fn put_u24(&mut self, value: u32) -> PutResult<B> {
        self._put(3, B::write_u24, value)
    }

    pub fn put_i24(&mut self, value: i32) -> PutResult<B> {
        self._put(3, B::write_i24, value)
    }

    pub fn put_u32(&mut self, value: u32) -> PutResult<B> {
        self._put(4, B::write_u32, value)
    }

    pub fn put_i32(&mut self, value: i32) -> PutResult<B> {
        self._put(4, B::write_i32, value)
    }

    pub fn put_f32(&mut self, value: f32) -> PutResult<B> {
        self._put(4, B::write_f32, value)
    }

    pub fn put_u48(&mut self, value: u64) -> PutResult<B> {
        self._put(6, B::write_u48, value)
    }

    pub fn put_i48(&mut self, value: i64) -> PutResult<B> {
        self._put(6, B::write_i48, value)
    }

    pub fn put_u64(&mut self, value: u64) -> PutResult<B> {
        self._put(8, B::write_u64, value)
    }

    pub fn put_i64(&mut self, value: i64) -> PutResult<B> {
        self._put(8, B::write_i64, value)
    }

    pub fn put_f64(&mut self, value: f64) -> PutResult<B> {
        self._put(8, B::write_f64, value)
    }

    pub fn put_u128(&mut self, value: u128) -> PutResult<B> {
        self._put(16, B::write_u128, value)
    }

    pub fn put_i128(&mut self, value: i128) -> PutResult<B> {
        self._put(16, B::write_i128, value)
    }

    pub fn put_slice(&mut self, value: &[u8]) -> PutResult<B> {
        self._put(value.len(), |buf, value| buf.copy_from_slice(value), value)
    }

    pub fn take<T: BufferTake>(&mut self) -> TakeResult<T> {
        T::take(self)
    }

    pub fn take_u8(&mut self) -> TakeResult<u8> {
        self._take(1, |buf| buf[0] as u8)
    }

    pub fn take_i8(&mut self) -> TakeResult<i8> {
        self._take(1, |buf| buf[0] as i8)
    }

    pub fn take_u16(&mut self) -> TakeResult<u16> {
        self._take(2, B::read_u16)
    }

    pub fn take_i16(&mut self) -> TakeResult<i16> {
        self._take(2, B::read_i16)
    }

    pub fn take_u24(&mut self) -> TakeResult<u32> {
        self._take(3, B::read_u24)
    }

    pub fn take_i24(&mut self) -> TakeResult<i32> {
        self._take(3, B::read_i24)
    }

    pub fn take_u32(&mut self) -> TakeResult<u32> {
        self._take(4, B::read_u32)
    }

    pub fn take_i32(&mut self) -> TakeResult<i32> {
        self._take(4, B::read_i32)
    }

    pub fn take_f32(&mut self) -> TakeResult<f32> {
        self._take(4, B::read_f32)
    }

    pub fn take_u48(&mut self) -> TakeResult<u64> {
        self._take(6, B::read_u48)
    }

    pub fn take_i48(&mut self) -> TakeResult<i64> {
        self._take(6, B::read_i48)
    }

    pub fn take_u64(&mut self) -> TakeResult<u64> {
        self._take(8, B::read_u64)
    }

    pub fn take_i64(&mut self) -> TakeResult<i64> {
        self._take(8, B::read_i64)
    }

    pub fn take_f64(&mut self) -> TakeResult<f64> {
        self._take(8, B::read_f64)
    }

    pub fn take_u128(&mut self) -> TakeResult<u128> {
        self._take(16, B::read_u128)
    }

    pub fn take_i128(&mut self) -> TakeResult<i128> {
        self._take(16, B::read_i128)
    }

    pub fn take_slice(&mut self, into: &mut [u8]) -> TakeResult<()> {
        self._take(into.len(), |buf| into.copy_from_slice(buf))
    }

    pub fn take_vec(&mut self) -> Vec<u8> {
        let mut vec = vec![0u8; self.get_remaining()];
        self.take_slice(&mut vec).map(|_| vec).unwrap()
    }

    fn _put<T>(&mut self, length: usize, writer: impl FnOnce(&mut [u8], T), value: T) -> PutResult<B> {
        let new_position = self.position + length;
        if new_position > self.limit {
            Err(BufferOverflowError {
                requested: length,
                remaining: self.get_remaining(),
            })
        } else {
            writer(&mut self.buffer[self.position..new_position], value);
            self.position = new_position;
            Ok(self)
        }
    }

    fn _take<T>(&mut self, length: usize, reader: impl FnOnce(&[u8]) -> T) -> TakeResult<T> {
        let new_position = self.position + length;
        if new_position > self.limit {
            Err(BufferUnderflowError {
                requested: length,
                remaining: self.get_remaining(),
            })
        } else {
            let value: T = reader(&self.buffer[self.position..new_position]);
            self.position = new_position;
            Ok(value)
        }
    }
}

impl<B: ByteOrder> Into<Vec<u8>> for ByteBuffer<B> {
    fn into(mut self) -> Vec<u8> {
        self.take_vec()
    }
}

impl<B: ByteOrder> Debug for ByteBuffer<B> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "ByteBuffer[pos={} lim={} cap={}]",
            self.position,
            self.limit,
            self.get_capacity()
        )
    }
}
