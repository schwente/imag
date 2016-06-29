use uuid::Uuid;

use libimagstore::store::Store;
use libimagstore::storeid::IntoStoreId;
use module_path::ModuleEntryPath;
use std::result::Result as RResult;

use error::{TodoError, TodoErrorKind};

pub type Result<T> = RResult<T, TodoError>;

/// With the uuid we get the storeid and then we can delete the entry
pub fn delete_func(store : &Store, uuid: Uuid) -> Result<()> {	
	// With the uuid we get the storeid
	let store_id = ModuleEntryPath::new(format!("taskwarrior/{}", uuid)).into_storeid();
	// It deletes an entry	
	match store.delete(store_id) {
		Ok(val) => Ok(val),
		Err(e) => Err(TodoError::new(TodoErrorKind::StoreError, Some(Box::new(e)))),	
	}
}

