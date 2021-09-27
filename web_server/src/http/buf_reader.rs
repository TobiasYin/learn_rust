use std::io::Read;
use std::io;
use crate::http::result::Res;

pub struct BufferReader<'a, T>
    where T: Read
{
    reader: &'a mut T,
    buf: Vec<u8>,
    pos: usize,
    line_sep: Vec<u8>,
    truck_size: usize,
    truck_buf: Vec<u8>,
}

impl<'a, T: Read> BufferReader<'a, T> {
    pub fn new(read: &mut T) -> BufferReader<T> {
        let default_truck_size = 1024usize;
        BufferReader {
            reader: read,
            buf: vec![],
            pos: 0,
            line_sep: vec!['\n' as u8],
            truck_size: default_truck_size,
            truck_buf: {
                let mut v = Vec::with_capacity(default_truck_size);
                v.resize(default_truck_size, 0);
                v
            },
        }
    }

    pub fn set_truck_size(&mut self, size: usize) {
        self.truck_size = size;
    }

    fn inner_read(&mut self) -> io::Result<()> {
        let size = self.reader.read(&mut self.truck_buf)?;
        self.buf.extend_from_slice(&self.truck_buf[0..size]);
        Ok(())
    }

    fn inner_read_with_size(&mut self, mut size: usize) -> io::Result<()> {
        if size > self.truck_size {
            size = self.truck_size
        }
        let size = self.reader.read(&mut self.truck_buf[0..size])?;
        self.buf.extend_from_slice(&self.truck_buf[0..size]);
        Ok(())
    }

    fn next_byte(&mut self) -> io::Result<u8> {
        if self.buf.len() <= self.pos {
            self.inner_read_with_size(self.truck_size)?;
        }
        if self.buf.len() <= self.pos {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "EOF"));
        }
        let c = self.buf[self.pos];
        self.pos += 1;
        Ok(c)
    }

    pub fn set_line_sep(&mut self, sep: &str) {
        self.line_sep = Vec::from(sep);
    }

    pub fn read_line(&mut self) -> Res<String> {
        let start = self.pos;
        if self.line_sep.len() == 0 {
            return Ok(String::from_utf8(vec![self.next_byte()?])?);
        }
        let mut sep_pos = 0;
        loop {
            let next = self.next_byte()?;
            if next == self.line_sep[sep_pos] {
                sep_pos += 1;
                if sep_pos >= self.line_sep.len() {
                    break;
                }
            } else {
                sep_pos = 0;
            }
        }

        Ok(String::from(std::str::from_utf8(&self.buf[start..(self.pos - self.line_sep.len())])?))
    }

    pub fn read_size(&mut self, size: usize) -> io::Result<Vec<u8>> {
        let mut left = self.buf.len() - self.pos;
        while left < size {
            self.inner_read_with_size(left)?;
            left = self.buf.len() - self.pos;
        }

        Ok(Vec::from(&self.buf[self.pos..(self.pos + size)]))
    }
}
