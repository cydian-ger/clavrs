use serde::{Serialize, Deserialize};
use std::collections::HashSet;
use std::fs;

use crate::connection::permission::Permission;

#[derive(Serialize, Deserialize, Debug)]
pub struct Permissions {
    permissions: HashSet<Permission>,
    default: Permission
}

impl Permissions {
    pub fn default() -> Permissions {
        Permissions { permissions: HashSet::new(), default: Permission::default() }
    }

    pub fn from_path(optional_path: Option<String>) -> Permissions {
        let path: String;
        match optional_path {
            Some(p_path) => {path = p_path}
            None => {return Permissions::default();}
        };

        let Ok(perm_string) = fs::read_to_string(path) else {return Permissions::default()};

        let json: Result<Permissions, _> = serde_json::from_str(&perm_string);

        return json.unwrap();
    }
}
