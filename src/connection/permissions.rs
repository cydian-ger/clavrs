use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;

use crate::connection::permission::Permission;

#[derive(Serialize, Deserialize, Debug)]
pub struct Permissions {
    permissions: HashSet<Permission>,
    default: Permission,
}

impl Permissions {
    pub fn default() -> Permissions {
        let admin_permissions = [
            ("read".to_string(), true),
            ("write".to_string(), true),
            ("transaction".to_string(), true),
            ("authenticate".to_string(), true),
        ]
        .iter()
        .cloned()
        .collect();
        Permissions {
            permissions: HashSet::from_iter(vec![Permission::root(), Permission::new("admin".to_string(), admin_permissions, "admin".to_string())]),
            default: Permission::default(),
        }
    }

    pub fn from_path(optional_path: Option<String>) -> Result<Permissions, String> {
        let path: String;
        match optional_path {
            Some(p_path) => path = p_path,
            None => {
                return Ok(Permissions::default());
            }
        };

        let Ok(perm_string) = fs::read_to_string(path) else {
            return Ok(Permissions::default());
        };

        let json: Result<Permissions, _> = serde_json::from_str(&perm_string);

        json.unwrap();
        return Err("Womp womp".to_string());
    }
}
