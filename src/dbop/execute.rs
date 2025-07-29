use std::sync::{Arc, Mutex};

use super::{
    op_type::Op, read::execute_read, read_write::execute_read_write, write::execute_write,
};
use evmap::{ReadHandle, WriteHandle};

pub fn execute_single(
    write_mutex: &Arc<Mutex<WriteHandle<String, String>>>,
    read_handle: &ReadHandle<String, String>,
    operation: Op,
) -> Result<String, String> {
    match operation {
        Op::Write(write_op) => {
            let mut write_handle = write_mutex.lock().unwrap();
            let ret = execute_write(&mut write_handle, write_op);
            write_handle.refresh();
            return ret;
        }

        Op::Read(read_op) => {
            return execute_read(&read_handle, read_op);
        }

        Op::ReadWrite(readwrite_op) => {
            let mut write_handle = write_mutex.lock().unwrap();
            let ret = execute_read_write(&mut write_handle, readwrite_op);
            write_handle.refresh();
            return ret;
        }
    };
}

pub fn execute_transaction(
    write_mutex: &Arc<Mutex<WriteHandle<String, String>>>,
    read_handle: &ReadHandle<String, String>,
    operations: Vec<Op>,
) -> Result<String, String> {
    let mut write_handle = write_mutex.lock().unwrap();

    // enumerate and have failure index and amount of completed commands and shit.
    for (index, operation) in operations.into_iter().enumerate() {
        let res;
        match operation {
            Op::Write(write_op) => {
                res = execute_write(&mut write_handle, write_op);
            }
            Op::Read(read_op) => {
                res = execute_read(&read_handle, read_op);
            }
            Op::ReadWrite(readwrite_op) => {
                res = execute_read_write(&mut write_handle, readwrite_op);
            }
        }

        match res {
            Ok(_) => {},
            Err(err) => {
                return Err(format!("{}){}", index, err))
            },
        }
    }

    write_handle.refresh();
    Ok("Ok".to_string())
}