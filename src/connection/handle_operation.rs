use crate::{lexer::{validate::Part, parse::parse_operation}, dbop::{op_type::Op, execute::execute_single}};
use super::{connection_state::{ConnectionState, OperationMode}, permission::Permission};

pub fn handle_operation(
    parts: Vec<Part>,
    connection_state: &mut ConnectionState,
    permission: &Permission,
) -> Result<String, String> {
    // Todo implement return and write for Success and Failure of Operation

    let operation: Op;

    match parse_operation(parts) {
        Ok(parsed_operation) => {
            operation = parsed_operation;
        }
        Err(err) => {
            return Err(err.to_string());
        }
    }

    // Check if the command is allowed
    permission.allow_operation(&operation)?;

    match connection_state.mode {
        OperationMode::Default => {
            return execute_single(
                &connection_state.write_mutex,
                &connection_state.read_handle,
                operation,
            )
        }
        OperationMode::Transaction => {
            // Queue new Operation here
            connection_state.op_queue.push(operation);
            return Ok("+Queue".to_string());
        }
    }
}