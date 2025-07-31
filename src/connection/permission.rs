use serde::{Deserialize, Serialize};

use crate::{
    connection::{connection_state::ConnectionState, permission_matrix::PermissionMatrix},
    dbop::op_type::{Instruction, Op, ReadOp, ReadWriteOp, WriteOp},
    Mode,
};

use std::{
    collections::HashMap, hash::{Hash, Hasher}
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission {
    pub name: String,
    permission_matrix: PermissionMatrix,
}

impl PartialEq for Permission {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl Eq for Permission {}

impl Hash for Permission {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl Permission {
    pub fn new(name: String, permission_map: HashMap<String, bool>) -> Self {
        return Permission {
            name: name,
            permission_matrix: PermissionMatrix::new(permission_map),
        };
    }

    pub fn default() -> Self {
        // The only allowed action should be authenticate
        let default_permissions: HashMap<String, bool> =
            [("read".to_string(), true), ("write".to_string(), true)]
                .iter()
                .cloned()
                .collect();
        return Permission {
            name: "default".to_string(),
            permission_matrix: PermissionMatrix::new(default_permissions),
        };
    }

    fn can_use_restricted(&self, mode: &Mode) -> Result<bool, String> {
        match mode {
            Mode::Test => {}
            _ => {
                return Err(
                    format!("Can not use restricted Commands in mode {:?}", mode).to_string(),
                );
            }
        }
        Ok(true)
    }

    pub fn allow_operation(
        &self,
        op: &Op,
        connection_state: &ConnectionState,
    ) -> Result<(), String> {
        // if !connection_state.is_authenticated {
        //     return Err("Authentication is required before issuing operations.".to_string());
        // };

        let possible = match op {
            Op::Read(read) => match read {
                ReadOp::Get(_) => self.permission_matrix.get_permission("read.get"),
                ReadOp::Exists(_) => self.permission_matrix.get_permission("read.exists"),
                ReadOp::Has(_, _) => self.permission_matrix.get_permission("read.has"),
            },
            Op::Write(write) => match write {
                WriteOp::Put(_, _) => self.permission_matrix.get_permission("write.put"),
                WriteOp::Delete(_) => self.permission_matrix.get_permission("write.delete"),
                WriteOp::Clear(_) => self.permission_matrix.get_permission("write.clear"),
                WriteOp::Replace(_, _) => self.permission_matrix.get_permission("write.replace"),
                WriteOp::Retract(_, _) => self.permission_matrix.get_permission("write.retract"),
                WriteOp::Purge => {
                    match self.can_use_restricted(&connection_state.db_mode) {
                        Ok(_) => self.permission_matrix.get_permission("write.pop"),
                        Err(err) => return Err(err),
                    }
                }
            },
            Op::ReadWrite(read_write) => match read_write {
                ReadWriteOp::Pop(_) => self.permission_matrix.get_permission("write.pop"),
            }
        };
        
        match possible {
            true => Ok(()),
            false => Err("Permissions are not sufficient to perform this operation".to_string()),
        }
    }

    pub fn allow_instruction(
        &self,
        instruction: &Instruction,
        connections_state: &ConnectionState,
    ) -> Result<(), String> {
        match instruction {
            Instruction::Transaction(_transaction) => {
                // if !connections_state.is_authenticated {
                //     return Err(
                //         "Authentication is required before issuing instructions.".to_string()
                //     );
                // }
                if self.permission_matrix.get_permission("transaction") {
                    return Ok(());
                };
            }
            Instruction::Authenticate(_authentication) => {
                if self.permission_matrix.get_permission("authenticate") {
                    return Ok(());
                };
            }
        }

        Err("Permissions are not sufficient to perform this instruction".to_string())
    }
}
