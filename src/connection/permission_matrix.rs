use std::collections::HashMap;

use serde::{Serialize, Deserialize};



#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Hash)]
pub struct PermissionMatrix {
    //
    read: Option<bool>,
    get: Option<bool>,
    exists: Option<bool>,
    has: Option<bool>,
    // Write
    write: Option<bool>,
    put: Option<bool>,
    delete: Option<bool>,
    clear: Option<bool>,
    replace: Option<bool>,
    retract: Option<bool>,
    purge: Option<bool>,
    // ReadWrite
    // readwrite: Option<bool>,
    pop: Option<bool>,
    // Transaction
    transaction: Option<bool>,
    // Authetnciate
    authenticate: Option<bool>,
}

impl Eq for PermissionMatrix {}

impl PermissionMatrix {
    pub fn new(matrix: HashMap<String, bool>) -> Self {
        let get = |key: &str| matrix.get(key).copied();

        Self {
            read: get("read"),
            get: get("read.get"),
            exists: get("read.exists"),
            has: get("read.has"),

            write: get("write"),
            put: get("write.put"),
            delete: get("write.delete"),
            clear: get("write.clear"),
            replace: get("write.replace"),
            retract: get("write.retract"),
            purge: get("write.purge"),

            pop: get("readwrite.pop"),

            transaction: get("transaction"),

            authenticate: get("authenticate"),
        }
    }

    pub fn get_permission(&self, permission: &str) -> bool {
        match permission {
            // READ
            "read" => self.read.unwrap_or(false),

            "read.get" => match self.get {
                Some(v) => v,
                None => self.read.unwrap_or(false),
            },
            "read.exists" => match self.exists {
                Some(v) => v,
                None => self.read.unwrap_or(false),
            },
            "read.has" => match self.has {
                Some(v) => v,
                None => self.read.unwrap_or(false),
            },

            // WRITE
            "write" => self.write.unwrap_or(false),

            "write.put" => match self.put {
                Some(v) => v,
                None => self.write.unwrap_or(false),
            },
            "write.delete" => match self.delete {
                Some(v) => v,
                None => self.write.unwrap_or(false),
            },
            "write.clear" => match self.clear {
                Some(v) => v,
                None => self.write.unwrap_or(false),
            },
            "write.replace" => match self.replace {
                Some(v) => v,
                None => self.write.unwrap_or(false),
            },
            "write.retract" => match self.retract {
                Some(v) => v,
                None => self.write.unwrap_or(false),
            },
            "write.purge" => match self.purge {
                Some(v) => v,
                None => self.write.unwrap_or(false),
            },

            // READ WRITE
            // "readwrite" => self.readwrite.unwrap_or(false),

            "write.pop" => match self.pop {
                Some(v) => v,
                None => {
                    // If both write and read are set
                    if self.write.unwrap_or(false) && self.read.unwrap_or(false) {
                        return true;
                    }
                    return false;
                },
            },

            "transaction" => match self.transaction {
                Some(v) => v,
                None => self.write.unwrap_or(false)
            },

            // Authenticate true by default
            "authenticate" => self.authenticate.unwrap_or(true),

            _ => false,
        }
    }

}
