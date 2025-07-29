use std::sync::{Arc, Mutex};

use crate::dbop::op_type::Op;
use evmap::{ReadHandle, WriteHandle};

#[derive(Debug)]
pub enum OperationMode {
    Default,
    Transaction,
}

pub struct ConnectionState {
    pub mode: OperationMode,
    pub op_queue: Vec<Op>,
    pub read_handle: ReadHandle<String, String>,
    pub write_mutex: Arc<Mutex<WriteHandle<String, String>>>,
}

impl ConnectionState {
    pub fn new(read_handle: ReadHandle<String, String>, write_mutex: Arc<Mutex<WriteHandle<String, String>>>) -> Self {
        ConnectionState { mode: OperationMode::Default, op_queue: Vec::new(), read_handle: read_handle, write_mutex: write_mutex }
    }
}