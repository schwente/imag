use std::convert::From;
use std::convert::Into;

use libimagstore::store::Entry;
use libimagstore::storeid::IntoStoreId;

use module_path::ModuleEntryPath;

pub type CounterName = String;

pub struct Counter {
    name: CounterName,
    value: u64,
}

impl Counter {

    pub fn new(name: CounterName, init: u64) -> Counter {
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

    pub fn value(&self) -> u64 {
        self.value
    }

}

impl<'a> From<&'a Entry> for Counter {

    fn from(e: &'a Entry) -> Counter {
        let hdr = e.get_header();

        let name = String::from("Unimplemented");
        let value = 0;

        Counter {
            name: name,
            value: value,
        };

        unimplemented!()
    }

}

impl Into<Entry> for Counter {

    fn into(self) -> Entry {
        let path    = ModuleEntryPath::new(self.name);
        let mut e   = Entry::new(path.into_storeid());
        let mut hdr = e.get_header_mut();

        unimplemented!()
    }
}

trait IntoCounterName {
    fn into_countername(self) -> CounterName;
}

impl IntoCounterName for StoreId {

    fn into_countername(self) -> CounterName {
        unimplemented!()
    }

}

trait FromCounterName<T> {
    fn from_countername(c: CounterName) -> T;
}

impl FromCounterName<StoreId> for CounterName {

    fn into_countername(self) -> StoreId {
        unimplemented!()
    }

}
