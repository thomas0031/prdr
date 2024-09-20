use crate::endian::Endianness;
use crate::error::SerializeError;

pub trait Serialize {
    fn serialize(&self, buffer: &mut [u8], endian: Endianness) -> Result<usize, SerializeError>;
}

impl Serialize for u8 {
    fn serialize(&self, buffer: &mut [u8], _: Endianness) -> Result<usize, SerializeError> {
        if buffer.is_empty() {
            return Err(SerializeError::BufferOverflow);
        }

        buffer[0] = *self;

        Ok(1)
    }
}

impl Serialize for i8 {
    fn serialize(&self, buffer: &mut [u8], _: Endianness) -> Result<usize, SerializeError> {
        if buffer.is_empty() {
            return Err(SerializeError::BufferOverflow);
        }

        buffer[0] = *self as u8;

        Ok(1)
    }
}

impl Serialize for u16 {
    fn serialize(&self, buffer: &mut [u8], endian: Endianness) -> Result<usize, SerializeError> {
        if buffer.len() < 2 {
            return Err(SerializeError::BufferOverflow);
        }

        match endian {
            Endianness::Little => buffer[..2].copy_from_slice(&self.to_le_bytes()),
            Endianness::Big => buffer[..2].copy_from_slice(&self.to_be_bytes()),
        }

        Ok(2)
    }
}

impl Serialize for i16 {
    fn serialize(&self, buffer: &mut [u8], endian: Endianness) -> Result<usize, SerializeError> {
        if buffer.len() < 2 {
            return Err(SerializeError::BufferOverflow);
        }

        match endian {
            Endianness::Little => buffer[..2].copy_from_slice(&self.to_le_bytes()),
            Endianness::Big => buffer[..2].copy_from_slice(&self.to_be_bytes()),
        }

        Ok(2)
    }
}

impl Serialize for u32 {
    fn serialize(&self, buffer: &mut [u8], endian: Endianness) -> Result<usize, SerializeError> {
        if buffer.len() < 4 {
            return Err(SerializeError::BufferOverflow);
        }

        match endian {
            Endianness::Little => buffer[..4].copy_from_slice(&self.to_le_bytes()),
            Endianness::Big => buffer[..4].copy_from_slice(&self.to_be_bytes()),
        }

        Ok(4)
    }
}

impl Serialize for i32 {
    fn serialize(&self, buffer: &mut [u8], endian: Endianness) -> Result<usize, SerializeError> {
        if buffer.len() < 4 {
            return Err(SerializeError::BufferOverflow);
        }

        match endian {
            Endianness::Little => buffer[..4].copy_from_slice(&self.to_le_bytes()),
            Endianness::Big => buffer[..4].copy_from_slice(&self.to_be_bytes()),
        }

        Ok(4)
    }
}

impl Serialize for u64 {
    fn serialize(&self, buffer: &mut [u8], endian: Endianness) -> Result<usize, SerializeError> {
        if buffer.len() < 8 {
            return Err(SerializeError::BufferOverflow);
        }

        match endian {
            Endianness::Little => buffer[..8].copy_from_slice(&self.to_le_bytes()),
            Endianness::Big => buffer[..8].copy_from_slice(&self.to_be_bytes()),
        }

        Ok(8)
    }
}

impl Serialize for i64 {
    fn serialize(&self, buffer: &mut [u8], endian: Endianness) -> Result<usize, SerializeError> {
        if buffer.len() < 8 {
            return Err(SerializeError::BufferOverflow);
        }

        match endian {
            Endianness::Little => buffer[..8].copy_from_slice(&self.to_le_bytes()),
            Endianness::Big => buffer[..8].copy_from_slice(&self.to_be_bytes()),
        }

        Ok(8)
    }
}

impl Serialize for f32 {
    fn serialize(&self, buffer: &mut [u8], endian: Endianness) -> Result<usize, SerializeError> {
        if buffer.len() < 4 {
            return Err(SerializeError::BufferOverflow);
        }

        match endian {
            Endianness::Little => buffer[..4].copy_from_slice(&self.to_le_bytes()),
            Endianness::Big => buffer[..4].copy_from_slice(&self.to_be_bytes()),
        }

        Ok(4)
    }
}

impl Serialize for f64 {
    fn serialize(&self, buffer: &mut [u8], endian: Endianness) -> Result<usize, SerializeError> {
        if buffer.len() < 8 {
            return Err(SerializeError::BufferOverflow);
        }

        match endian {
            Endianness::Little => buffer[..8].copy_from_slice(&self.to_le_bytes()),
            Endianness::Big => buffer[..8].copy_from_slice(&self.to_be_bytes()),
        }

        Ok(8)
    }
}

impl Serialize for bool {
    fn serialize(&self, buffer: &mut [u8], _: Endianness) -> Result<usize, SerializeError> {
        if buffer.is_empty() {
            return Err(SerializeError::BufferOverflow);
        }

        buffer[0] = *self as u8;

        Ok(1)
    }
}

// TODO char

impl<T: Serialize> Serialize for Option<T> {
    fn serialize(&self, buffer: &mut [u8], endian: Endianness) -> Result<usize, SerializeError> {
        if buffer.is_empty() {
            return Err(SerializeError::BufferOverflow);
        }

        match self {
            Some(value) => {
                let size = value.serialize(&mut buffer[1..], endian)?;
                buffer[0] = 1;
                Ok(size + 1)
            }
            None => {
                buffer[0] = 0;
                Ok(1)
            }
        }
    }
}

const VEC_LENGTH_SIZE: usize = 4;
impl<T: Serialize> Serialize for Vec<T> {
    // TODO: think about max size of Vec
    fn serialize(&self, buf: &mut [u8], endian: Endianness) -> Result<usize, SerializeError> {
        let len = self.len() as u32;
        let len_size = len.serialize(&mut buf[..VEC_LENGTH_SIZE], endian)?;

        let mut offset = len_size;
        for item in self {
            let used = item.serialize(&mut buf[offset..], endian)?;
            offset += used;
        }

        Ok(offset)
    }
}

impl Serialize for String {
    fn serialize(&self, buf: &mut [u8], endian: Endianness) -> Result<usize, SerializeError> {
        self.as_bytes().to_vec().serialize(buf, endian) // TODO remove vec allocation
    }
}

// TODO implement arrays, tuples
