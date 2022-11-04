use std::io::Cursor;

use bytes::BytesMut;
use tokio::{io::{BufWriter, AsyncReadExt}, net::TcpStream};

#[derive(Debug)]
pub enum Error {
  Incomplete,
  Other(crate::Error),
}

pub(crate) struct Connection {
  stream: BufWriter<TcpStream>,
  buffer: BytesMut,
}

impl Connection {
  pub fn new(socket: TcpStream) -> Connection {
    Connection {
      stream: BufWriter::new(socket),
      buffer: BytesMut::with_capacity(4 * 1024),
    }
  }

  pub async fn read_frame(&mut self) -> Result<Option<String>, Error> {
    self.stream.read_buf(&mut self.buffer).await.unwrap();
    let mut cursor = Cursor::new(&self.buffer[..]);

    let line = get_line(&mut cursor)?.to_vec();
    let string = String::from_utf8(line).unwrap();

    self.buffer.clear();
    return Ok(Some(string));
  }
}

fn get_line<'a>(cursor: &mut Cursor<&'a [u8]>) -> Result<&'a [u8], Error> {
  let start = cursor.position() as usize;
  let end = cursor.get_ref().len() - 1;

  for i in start..end {
    if cursor.get_ref()[i] == b'\r' && cursor.get_ref()[i+1] == b'\n' {
      cursor.set_position((i+2) as u64);

      return Ok(&cursor.get_ref()[start..i]);
    }
  }

  Err(Error::Incomplete)
}
