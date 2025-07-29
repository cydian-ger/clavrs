use crate::{lexer::{validate::Part, parse::parse_instruction}, dbop::{op_type::{Instruction, TransactionOp, Op}, execute::execute_transaction}};

use super::{connection_state::{ConnectionState, OperationMode}, permission::Permission};

pub fn handle_instruction(
    parts: Vec<Part>,
    connection_state: &mut ConnectionState,
    permission: &Permission,
) -> Result<String, String> {
    let instruction: Instruction;

    match parse_instruction(parts) {
        Ok(parsed_instruction) => instruction = parsed_instruction,
        Err(err) => return Err(err.to_string()),
    }

    permission.allow_instruction(&instruction)?;

    match instruction {
        Instruction::Transaction(transaction) => {
            let mode_is_sequence: bool = match connection_state.mode {
                OperationMode::Default => false,
                _ => true,
            };

            match transaction {
                TransactionOp::Sequence => {
                    if mode_is_sequence {
                        return Err("Connection is already in sequence mode.".to_string());
                    }
                    connection_state.mode = OperationMode::Transaction;
                }
                TransactionOp::Abort => {
                    if !mode_is_sequence {
                        return Err("Connection is not in sequence mode.".to_string());
                    }

                    connection_state.op_queue.clear();
                    connection_state.mode = OperationMode::Default;
                }
                TransactionOp::Execute => {
                    if !mode_is_sequence {
                        return Err("Connection is not in sequence mode.".to_string());
                    }

                    // Drain the old vector into the new vector to pass it along
                    let operations: Vec<Op> =
                        connection_state.op_queue.drain(..).into_iter().collect();
                    connection_state.mode = OperationMode::Default;
                    return execute_transaction(
                        &connection_state.write_mutex,
                        &connection_state.read_handle,
                        operations,
                    );
                }
            }
        }
    }

    Ok("Ok".to_string())
}