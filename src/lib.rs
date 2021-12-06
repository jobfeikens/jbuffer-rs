mod byte_buffer;
pub use byte_buffer::*;

mod traits;
pub use traits::*;


#[cfg(test)]
mod tests {
    use std::fmt::Debug;
    use byteorder::NativeEndian;
    use crate::{BufferPut, BufferTake, ByteBuffer};

    #[test]
    fn it_works() {
        test_read_write(3u16);
    }

    fn test_read_write<T>(value: T) where T: BufferPut + BufferTake + PartialEq + Debug + Copy {
        let mut buffer = ByteBuffer::<NativeEndian>::allocate(1024);

        buffer.put(value).unwrap();

        let length = buffer.get_position();

        buffer.flip();

        let taken = buffer.take::<T>().unwrap();

        assert_eq!(length, buffer.get_position());
        assert_eq!(value, taken);
    }
}
