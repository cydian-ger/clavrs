use serde::{Serialize, Deserialize};
use std::fs;

use crate::Mode;
use super::permission::Permission;

#[derive(Serialize, Deserialize, Debug)]
pub struct PermissionList {
    permissions: Vec<PermissionState>,
    default: PermissionState
}

impl PermissionList {
    pub fn default() -> PermissionList {
        PermissionList { permissions: vec![], default: PermissionState::default() }
    }

    pub fn from_path(optional_path: Option<String>) -> PermissionList {
        let path: String;
        match optional_path {
            Some(p_path) => {path = p_path}
            None => {return PermissionList::default();}
        };

        let Ok(perm_string) = fs::read_to_string(path) else {return PermissionList::default()};

        let json: Result<PermissionList, _> = serde_json::from_str(&perm_string);

        return json.unwrap();
    }

    pub fn permission_from_auth_str(&self, token: String, ip: String, mode: &Mode) -> Permission {
        for permission in self.permissions.iter() {
            if let Some(permission) = permission.match_into(&token, &ip, mode.clone()) {
                return permission
            }
        }

        return Permission::new(&self.default, mode.clone());
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PermissionState {
    pub name: String,
    pub token: String
}

impl PermissionState {
    pub fn default() -> Self {
        PermissionState { name: "default".to_string(), token: "".to_string() }
    }

    pub fn match_into(&self, token: &String, ip: &String, mode: Mode) -> Option<Permission> {
        if token != &self.token {
            return None
        }

        return Some(Permission::new(&self, mode));
    }
}