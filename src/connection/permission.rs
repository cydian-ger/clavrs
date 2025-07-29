use crate::{
    dbop::op_type::{Op, WriteOp, Instruction},
    Mode,
};

use super::permission_list::PermissionState;

#[derive(Debug)]
pub struct Permission {
    pub name: String,
    mode: Mode,
}

impl Permission {
    pub fn new(permission_state: &PermissionState, mode: Mode) -> Self {
        return Permission {
            name: permission_state.name.clone(),
            mode: mode,
        };
    }

    fn can_read(&self) -> Result<(), String> {
        Ok(())
    }

    fn can_write(&self) -> Result<(), String> {
        Ok(())
    }

    fn can_transaction(&self) -> Result<(), String> {
        Ok(())
    }

    fn can_use_restricted(&self) -> Result<(), String> {
        match self.mode {
            Mode::Test => {}
            _ => {
                return Err(format!("Can not use restricted Commands in mode {:?}", self.mode).to_string());
            }
        }

        Ok(())
    }

    pub fn allow_operation(&self, op: &Op) -> Result<(), String> {
        match op {
            Op::Read(_read) => {
                self.can_read()?;
            }
            Op::Write(write) => {
                self.can_write()?;

                match write {
                    WriteOp::Purge => {
                        self.can_use_restricted()?;
                    }
                    _ => {}
                }
            }
            Op::ReadWrite(_read_write) => {
                self.can_read()?;
                self.can_write()?;
            }
        }

        Ok(())
    }

    pub fn allow_instruction(&self, instruction: &Instruction) -> Result<(), String> {
        match instruction {
            Instruction::Transaction(_transaction) => {
                self.can_transaction()?;
            }
        }

        Ok(())
    }
}
