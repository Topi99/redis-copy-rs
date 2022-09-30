use bytes::{BytesMut};
use tokio::{
  net::{TcpListener, TcpStream},
  sync::Semaphore,
  time,
  io::{BufReader, AsyncReadExt},
};
use std::{sync::Arc, time::Duration};

use crate::{db::{DbDropGuard, Db}, Result};

const MAX_CONNECTIONS: usize = 250;

pub async fn run(listener: TcpListener) {
  print!("Hola");
  let server = Server {
    listener,
    db_holder: DbDropGuard::new(),
    limit_connections: Arc::new(Semaphore::new(MAX_CONNECTIONS)),
  };

  server.run().await.unwrap();
}

struct Server {
  db_holder: DbDropGuard,
  listener: TcpListener,
  limit_connections: Arc<Semaphore>,
}

struct Handler {
  db: Db,
}

impl Server {
  async fn run(&self) -> Result<()> {
    println!("Ready to receive connections...");

    loop {
      let permit = self
        .limit_connections.clone().acquire_owned().await.unwrap();
      let mut socket = self.accept().await?;
      println!("Connection accepted.");

      tokio::spawn(async move {
        let (read, _) = socket.split();
        let mut reader = BufReader::new(read);
        let mut buff = BytesMut::with_capacity(1024);

        loop {
          reader.read_buf(&mut buff).await.unwrap();
          println!("{:?}", buff);
          buff.clear();
        }
      });
    }
  }

  async fn accept(&self) -> Result<TcpStream> {
    let mut backoff = 1;

    loop {
      match self.listener.accept().await {
        Ok((socket, _)) => return Ok(socket),
        Err(err) => {
          if backoff > 64 {
            return Err(err.into());
          }
        }
      }

      time::sleep(Duration::from_secs(backoff)).await;

      backoff *= 2;
    }
  }
}

impl Handler {
  pub async fn run(&self) -> Result<()> {
    loop {
      
    }
  }
}
