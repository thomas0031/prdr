use crate::endian::Endianness;
use crate::error::DeserializeError;

pub trait Deserialize: Sized {
    fn deserialize(buf: &[u8], endian: Endianness) -> Result<(Self, usize), DeserializeError>;
}

impl Deserialize for u8 {
    fn deserialize(buf: &[u8], _: Endianness) -> Result<(Self, usize), DeserializeError> {
        if buf.is_empty() {
            return Err(DeserializeError::Invalid);
        }

        Ok((buf[0], 1))
    }
}

impl Deserialize for i8 {
    fn deserialize(buf: &[u8], _: Endianness) -> Result<(Self, usize), DeserializeError> {
        if buf.is_empty() {
            return Err(DeserializeError::Invalid);
        }

        Ok((buf[0] as i8, 1))
    }
}

impl Deserialize for u16 {
    fn deserialize(buf: &[u8], endian: Endianness) -> Result<(Self, usize), DeserializeError> {
        if buf.len() < 2 {
            return Err(DeserializeError::Invalid);
        }

        let value = match endian {
            Endianness::Little => u16::from_le_bytes([buf[0], buf[1]]),
            Endianness::Big => u16::from_be_bytes([buf[0], buf[1]]),
        };

        Ok((value, 2))
    }
}

impl Deserialize for i16 {
    fn deserialize(buf: &[u8], endian: Endianness) -> Result<(Self, usize), DeserializeError> {
        if buf.len() < 2 {
            return Err(DeserializeError::Invalid);
        }

        let value = match endian {
            Endianness::Little => i16::from_le_bytes([buf[0], buf[1]]),
            Endianness::Big => i16::from_be_bytes([buf[0], buf[1]]),
        };

        Ok((value, 2))
    }
}

impl Deserialize for u32 {
    fn deserialize(buf: &[u8], endian: Endianness) -> Result<(Self, usize), DeserializeError> {
        if buf.len() < 4 {
            return Err(DeserializeError::Invalid);
        }

        let value = match endian {
            Endianness::Little => u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]),
            Endianness::Big => u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]),
        };

        Ok((value, 4))
    }
}

impl Deserialize for i32 {
    fn deserialize(buf: &[u8], endian: Endianness) -> Result<(Self, usize), DeserializeError> {
        if buf.len() < 4 {
            return Err(DeserializeError::Invalid);
        }

        let value = match endian {
            Endianness::Little => i32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]),
            Endianness::Big => i32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]),
        };

        Ok((value, 4))
    }
}

impl Deserialize for u64 {
    fn deserialize(buf: &[u8], endian: Endianness) -> Result<(Self, usize), DeserializeError> {
        if buf.len() < 8 {
            return Err(DeserializeError::Invalid);
        }

        let value = match endian {
            Endianness::Little => u64::from_le_bytes([
                buf[0], buf[1], buf[2], buf[3], buf[4], buf[5], buf[6], buf[7],
            ]),
            Endianness::Big => u64::from_be_bytes([
                buf[0], buf[1], buf[2], buf[3], buf[4], buf[5], buf[6], buf[7],
            ]),
        };

        Ok((value, 8))
    }
}

impl Deserialize for i64 {
    fn deserialize(buf: &[u8], endian: Endianness) -> Result<(Self, usize), DeserializeError> {
        if buf.len() < 8 {
            return Err(DeserializeError::Invalid);
        }

        let value = match endian {
            Endianness::Little => i64::from_le_bytes([
                buf[0], buf[1], buf[2], buf[3], buf[4], buf[5], buf[6], buf[7],
            ]),
            Endianness::Big => i64::from_be_bytes([
                buf[0], buf[1], buf[2], buf[3], buf[4], buf[5], buf[6], buf[7],
            ]),
        };

        Ok((value, 8))
    }
}

impl Deserialize for f32 {
    fn deserialize(buf: &[u8], endian: Endianness) -> Result<(Self, usize), DeserializeError> {
        if buf.len() < 4 {
            return Err(DeserializeError::Invalid);
        }

        let value = match endian {
            Endianness::Little => f32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]),
            Endianness::Big => f32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]),
        };

        Ok((value, 4))
    }
}

impl Deserialize for f64 {
    fn deserialize(buf: &[u8], endian: Endianness) -> Result<(Self, usize), DeserializeError> {
        if buf.len() < 8 {
            return Err(DeserializeError::Invalid);
        }

        let value = match endian {
            Endianness::Little => f64::from_le_bytes([
                buf[0], buf[1], buf[2], buf[3], buf[4], buf[5], buf[6], buf[7],
            ]),
            Endianness::Big => f64::from_be_bytes([
                buf[0], buf[1], buf[2], buf[3], buf[4], buf[5], buf[6], buf[7],
            ]),
        };

        Ok((value, 8))
    }
}

impl Deserialize for bool {
    fn deserialize(buf: &[u8], _: Endianness) -> Result<(Self, usize), DeserializeError> {
        if buf.is_empty() {
            return Err(DeserializeError::Invalid);
        }

        Ok((buf[0] != 0, 1))
    }
}

impl<T: Deserialize> Deserialize for Option<T> {
    fn deserialize(buf: &[u8], endian: Endianness) -> Result<(Self, usize), DeserializeError> {
        if buf.is_empty() {
            return Err(DeserializeError::Invalid);
        }

        match buf[0] {
            0 => Ok((None, 1)),
            1 => {
                let (value, size) = T::deserialize(&buf[1..], endian)?;
                Ok((Some(value), size + 1))
            }
            _ => Err(DeserializeError::Invalid),
        }
    }
}

const VEC_LENGTH_SIZE: usize = 4;
impl<T: Deserialize> Deserialize for Vec<T> {
    fn deserialize(buf: &[u8], endian: Endianness) -> Result<(Self, usize), DeserializeError> {
        if buf.len() < VEC_LENGTH_SIZE {
            return Err(DeserializeError::Invalid);
        }

        let length = u32::deserialize(&buf[..VEC_LENGTH_SIZE], endian)?.0 as usize;
        let mut vec = Vec::with_capacity(length); // TODO: think about performance if length is
                                                  // huge & payload is invalid
        let mut offset = 0;
        for _ in 0..length {
            let (value, size) = T::deserialize(&buf[VEC_LENGTH_SIZE + offset..], endian)?;
            vec.push(value);
            offset += size;
        }

        Ok((vec, offset + VEC_LENGTH_SIZE))
    }
}

impl Deserialize for String {
    fn deserialize(buf: &[u8], endian: Endianness) -> Result<(Self, usize), DeserializeError> {
        let (vec, size) = Vec::<u8>::deserialize(buf, endian)?;
        Ok((
            String::from_utf8(vec).map_err(|_| DeserializeError::Invalid)?,
            size,
        ))
    }
}

impl<T: Deserialize, const N: usize> Deserialize for [T; N] {
    fn deserialize(buffer: &[u8], endian: Endianness) -> Result<(Self, usize), DeserializeError> {
        let mut result = std::mem::MaybeUninit::<[T; N]>::uninit();
        let mut total_size = 0;

        for i in 0..N {
            let (item, size) = T::deserialize(&buffer[total_size..], endian)?;
            // SAFETY: We're writing to the i-th element, which is within bounds.
            unsafe {
                result.as_mut_ptr().cast::<T>().add(i).write(item);
            }
            total_size += size;
        }

        // SAFETY: All elements have been initialized.
        let result = unsafe { result.assume_init() };
        Ok((result, total_size))
    }
}
