use std::collections::HashSet;

use evmap::WriteHandle;
use super::op_type::*;

pub fn execute_write(write_handle: &mut WriteHandle<String, String>, operation: WriteOp) -> Result<String, String> {
    // write_handle.map_into(f) read all for dump

    match operation {
        // Put operations
        WriteOp::Put(keys, values_list) => {
            for (key, values) in keys.into_iter().zip(values_list.into_iter()) {
                // could reserve for optimization
                for value in values {
                    write_handle.insert(key.clone(), value);
                }
            }
        }

        // Delete operations
        WriteOp::Delete(keys) => {
            for key in keys.into_iter() {
                write_handle.empty(key);
            }
        }

        // Clear operation
        WriteOp::Clear(keys) => {
            for key in keys.into_iter() {
                // only clear key if it exists else there would be problems with lifetimes
                if write_handle.contains_key(&key) {
                    write_handle.clear(key);
                }
            }
        }

        // Retract operation
        WriteOp::Retract(keys, retract_values) => {
            let retract_value_set : HashSet<String> = HashSet::from_iter(retract_values.iter().map(|x| x.to_owned()));
            for key in keys.into_iter() {
                // Skip if no key is found
                if write_handle.get(&key).is_none() {
                    continue
                }

                // Need to copy here so it can be moved into the closure
                let retracte_value_set_copy = retract_value_set.clone();

                // Remove any values not specified
                unsafe {
                    write_handle.retain(key.clone(), move |x, _| !retracte_value_set_copy.contains(x));
                }
            }
        }

        WriteOp::Replace(key, values) => {
            if values.len() == 1 {
                write_handle.update(key, values.into_iter().collect());
            } else {
                // Remove all values if the key does exists
                if write_handle.contains_key(&key) {
                    write_handle.clear(key.clone());
                }
                // reserve the values
                write_handle.reserve(key.clone(), values.len());
                for value in values {
                    write_handle.insert(key.clone(), value);
                }
            }
        }

        WriteOp::Purge => {
            write_handle.purge();
        }
    }

    return Ok(String::from("Ok"));
}
