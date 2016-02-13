use std::convert::Into;

use libimagstore::store::Entry;

pub struct Note {
    text: String,
}

impl Note {

    pub fn new(text: String) -> Note {
        Note {
            text: text
        }
    }

}

impl Into<Entry> for Note {

    fn into(self) -> Entry {
        unimplemented!()
    }

}

