use std::io::Read;

use libimagrt::runtime::Runtime;

use note::Note;
use result::Result;

struct NoteBuilder<'a> {
    rt: &'a Runtime<'a>,
    note: Note
}

impl<'a> NoteBuilder<'a> {

    pub fn new(rt: &'a Runtime) -> NoteBuilder<'a> {
        unimplemented!()
    }

    pub fn with_text_from<R: Read>(self, r: R) -> NoteBuilder<'a> {
        unimplemented!()
    }

    pub fn with_text(self, t: String) -> NoteBuilder<'a> {
        unimplemented!()
    }

    pub fn with_tags_from<R: Read>(self, r: R, sep: char) -> NoteBuilder<'a> {
        unimplemented!()
    }

    pub fn with_tags(self, t: Vec<String>) -> NoteBuilder<'a> {
        unimplemented!()
    }

    pub fn build(self) -> Result<Note> {
        unimplemented!()
    }

}

