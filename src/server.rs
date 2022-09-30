use bytes::Bytes;
use tokio::{net::{TcpListener, TcpStream}, sync::Semaphore, time};
use std::{str, sync::Arc, time::Duration};

use crate::{db::DbDropGuard, Result};

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

impl Server {
  async fn run(&self) -> Result<()> {
    loop {
      let permit = self
        .limit_connections.clone().acquire_owned().await.unwrap();

      println!("Ready to receive connections...");
      self.accept().await?;

      println!("Accepted connection");
      // drop(permit);
    }
  }

  async fn accept(&self) -> Result<TcpStream> {
      // time::sleep(Duration::from_secs(1)).await;
      // Ok(())
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
