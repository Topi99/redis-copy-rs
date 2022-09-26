use bytes::Bytes;
use tokio::net::TcpListener;
use std::str;

use crate::db::DbDropGuard;

pub async fn run(listener: TcpListener) {
  let db = DbDropGuard::new().db();
  let data = Bytes::from("Mundo");
  db.set("Hola".to_owned(), data);
  let result = db.get("Hola").unwrap();
  let parsed = str::from_utf8(&result).unwrap();
  println!("{:?}", parsed);
}
