use toml::Value;

use libimagstore::store::Entry;

use result::Result;
use tag::Tag;
use error::{TagError, TagErrorKind};

pub fn add_tag(e: &mut Entry, t: &Tag) -> Result<()> {
}