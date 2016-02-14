use std::convert::From;
use std::convert::Into;

use toml::Value;

use libimagstore::store::Store;
use libimagstore::storeid::StoreId;
use libimagstore::error::StoreError;
use libimagstore::store::Entry;
use libimagstore::storeid::IntoStoreId;

use module_path::ModuleEntryPath;

pub type CounterName = String;

pub struct Counter {
    name: CounterName,
    value: i64,
}

impl Counter {

    pub fn new(name: CounterName, init: i64) -> Counter {
        Counter {
            name: name,
            value: init,
        }
    }

    pub fn inc(&mut self) {
        self.value = self.value + 1;
    }

    pub fn name(&self) -> &CounterName {
        &self.name
    }

    pub fn value(&self) -> i64 {
        self.value
    }

    pub fn persist(self, store: &Store) -> Result<(), StoreError> {
        use std::ops::DerefMut;

        let mut lockentry = store.create(ModuleEntryPath::new(self.name.clone()).into_storeid());
        if lockentry.is_err() {
            return Err(lockentry.err().unwrap());
        }
        let mut lockentry = lockentry.unwrap();

        let mut entry  = lockentry.deref_mut();
        let mut header = entry.get_header_mut();
        let setres = header.set("counter.name", Value::String(self.name));
        if setres.is_err() {
            return Err(setres.err().unwrap());
        }

        let setres = header.set("counter.value", Value::Integer(self.value));
        if setres.is_err() {
            return Err(setres.err().unwrap());
        }

        Ok(())
    }

}

