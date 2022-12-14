use std::{sync::{Arc, Mutex}, collections::HashMap};

use bytes::Bytes;

#[derive(Debug)]
pub(crate) struct DbDropGuard {
  db: Db,
}

#[derive(Debug, Clone)]
pub struct Db {
  shared: Arc<Shared>,
}

#[derive(Debug)]
pub struct Shared {
  state: Mutex<State>,
}

#[derive(Debug)]
pub struct State {
  entries: HashMap<String, Entry>,
}

#[derive(Debug, Clone)]
pub struct Entry {
  data: Bytes,
}

impl DbDropGuard {
  pub(crate) fn new() -> DbDropGuard {
    DbDropGuard { db: Db::new() }
  }

  pub(crate) fn db(&self) -> Db {
    self.db.clone()
  }
}

impl Db {
  pub(crate) fn new() -> Db {
    let shared = Arc::new(Shared {
      state: Mutex::new(State {
        entries: HashMap::new(),
      }),
    });
    Db { shared }
  }

  pub(crate) fn get(&self, key: &str) -> Option<Bytes> {
    let state = self.shared.state.lock().unwrap();
    state.entries.get(key).map(|entry| entry.data.clone())
  }

  pub(crate) fn set(&self, key: String, data: Bytes) {
    let mut state = self.shared.state.lock().unwrap();
    state.entries.insert(key, Entry { data });
  }
}
