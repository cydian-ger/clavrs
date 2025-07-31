use super::{
    token::{Keyword, KeywordType},
    validate::Part,
};
use crate::dbop::op_type::{Instruction, Op, ReadOp, ReadWriteOp, WriteOp, TransactionOp};

pub fn parse_operation(parts: Vec<Part>) -> Result<Op, &'static str> {
    match parts.get(0).unwrap() {
        Part::Keyword {
            keyword,
            keyword_type,
        } => {
            match keyword_type {
                // Commands
                KeywordType::Operation => {
                    match keyword {
                        // Read
                        Keyword::GET => return parse_get(parts),
                        Keyword::HAS => return parse_has(parts),
                        Keyword::EXISTS => return parse_exists(parts),
                        // Write
                        Keyword::PUT => return parse_put(parts),
                        Keyword::DELETE => return parse_delete(parts),
                        Keyword::CLEAR => return parse_clear(parts),
                        Keyword::REPLACE => return parse_replace(parts),
                        Keyword::RETRACT => return parse_retract(parts),
                        // Restricted-Write
                        Keyword::PURGE => return parse_purge(parts),
                        // Read-Write
                        Keyword::POP => return parse_pop(parts),
                        _ => {},
                    }
                }

                _ => {}
            }
            return Err("Operation does not exist");
        }
        _ => return Err("First Argument needs to be Keyword"),
    }
}

pub fn parse_instruction(parts: Vec<Part>) -> Result<Instruction, &'static str> {

    match parts.get(0).unwrap() {
        Part::Keyword { keyword, keyword_type } => {
            match keyword_type {
                KeywordType::Instruction => {
                    match keyword {
                        // Transaction
                        Keyword::SEQEUENCE => {return Ok(Instruction::Transaction(TransactionOp::Sequence))},
                        Keyword::ABORT => {return Ok(Instruction::Transaction(TransactionOp::Abort))},
                        Keyword::EXECUTE => {return Ok(Instruction::Transaction(TransactionOp::Execute))},
                        // Auth
                        Keyword::AUTH => {return parse_auth(parts)}
                        _ => {}
                    }
                    return Err("Instruction does not exist")
                },

                _ => {}
            }
        }
        _ => {return Err("Function not implemented")}
    }
    return Err("A")
}

macro_rules! load_or_err {
    ($target:ident, $val:expr) => {
        match $val {
            Ok(val) => $target = val,
            Err(err) => return Err(err),
        }
    };
}

macro_rules! match_into {
    (
        $func_name:ident, $val:ident, $p:path, $into:ty
    ) => {
        pub fn $func_name($val: Option<&Part>) -> Result<$into, &'static str> {
            match $val {
                Some(inner_val) => match inner_val {
                    $p { values } => return Ok(values.to_vec()),
                    _ => return Err("DEV Failed to match appropriately"),
                },
                None => return Err("DEV Part is empty."),
            };
        }
    };
}

match_into!(match_into_values, vec, Part::Values, Vec<String>);
match_into!(match_into_nested, vec, Part::NestedValues, Vec<Vec<String>>);

fn match_into_value(val: Option<&Part>) -> Result<String, &'static str> {
    match val {
        Some(inner_val) => match inner_val {
            Part::Value { value } => return Ok(value.to_string()),
            _ => return Err("DEV Failed to match into single value."),
        },
        None => return Err("DEV match into value part was not found."),
    }
}

// -- READ --
// GET [KEYS]
fn parse_get(parts: Vec<Part>) -> Result<Op, &'static str> {
    let keys: Vec<String>;
    if parts.len() != 2 {
        return Err("GET requires 1 Argument: <Keys>");
    }

    load_or_err!(keys, match_into_values(parts.get(1)));

    return Ok(Op::Read(ReadOp::Get(keys)));
}

// HAS [KEYS] VALUE
fn parse_has(parts: Vec<Part>) -> Result<Op, &'static str> {
    let keys: Vec<String>;
    let value: String;
    if parts.len() != 3 {
        return Err("HAS requires 2 Arguments: <Keys> <Value>");
    }

    load_or_err!(keys, match_into_values(parts.get(1)));
    load_or_err!(value, match_into_value(parts.get(2)));

    return Ok(Op::Read(ReadOp::Has(keys, value)));
}

// EXISTS [KEYS]
fn parse_exists(parts: Vec<Part>) -> Result<Op, &'static str> {
    let keys: Vec<String>;
    if parts.len() != 2 {
        return Err("EXISTS requires 1 Argument: <Keys>");
    }

    load_or_err!(keys, match_into_values(parts.get(1)));

    return Ok(Op::Read(ReadOp::Exists(keys)));
}

// -- WRITE --
// PUT [KEYS] [[VALUES]]
fn parse_put(parts: Vec<Part>) -> Result<Op, &'static str> {
    let keys: Vec<String>;
    let values: Vec<Vec<String>>;
    if parts.len() != 3 {
        return Err("PUT requires 2 Arguments: <Keys> <<Values>>");
    }

    load_or_err!(keys, match_into_values(parts.get(1)));
    load_or_err!(values, match_into_nested(parts.get(2)));

    if keys.len() != values.len() {
        return Err("Amount of Keys must match amount of values provided.");
    }

    return Ok(Op::Write(WriteOp::Put(keys, values)));
}

// DELETE [KEYS]
fn parse_delete(parts: Vec<Part>) -> Result<Op, &'static str> {
    let keys;
    if parts.len() != 2 {
        return Err("DELETE requires 1 Argument: <Keys>");
    }

    load_or_err!(keys, match_into_values(parts.get(1)));

    return Ok(Op::Write(WriteOp::Delete(keys)));
}

// CLEAR [KEYS]
fn parse_clear(parts: Vec<Part>) -> Result<Op, &'static str> {
    let keys;
    if parts.len() != 2 {
        return Err("CLEAR requires 1 Argument: <Keys>");
    }

    load_or_err!(keys, match_into_values(parts.get(1)));

    return Ok(Op::Write(WriteOp::Clear(keys)));
}

// REPLACE [KEYS] [[VALUES]]
fn parse_replace(parts: Vec<Part>) -> Result<Op, &'static str> {
    let key;
    let values;

    if parts.len() != 3 {
        return Err("REPLACE requires 2 Arguments <Keys> <Values>");
    }

    load_or_err!(key, match_into_value(parts.get(1)));
    load_or_err!(values, match_into_values(parts.get(2)));

    return Ok(Op::Write(WriteOp::Replace(key, values)));
}

// RETRACT [KEYS] [[VALUES]]
fn parse_retract(parts: Vec<Part>) -> Result<Op, &'static str> {
    let keys;
    let values;

    if parts.len() != 3 {
        return Err("RETRACT requires 2 Arguments: <Keys> <Values>");
    }

    load_or_err!(keys, match_into_values(parts.get(1)));
    load_or_err!(values, match_into_values(parts.get(2)));

    return Ok(Op::Write(WriteOp::Retract(keys, values)));
}

// PURGE
fn parse_purge(parts: Vec<Part>) -> Result<Op, &'static str> {
    if parts.len() != 1 {
        return Err("PURGE requires no Arguments");
    }

    return Ok(Op::Write(WriteOp::Purge));
}

// POP KEY
fn parse_pop(parts: Vec<Part>) -> Result<Op, &'static str> {
    let key: String;

    if parts.len() != 2 {
        return Err("POP requires 1 Argument: <Key>");
    }

    load_or_err!(key, match_into_value(parts.get(1)));

    return Ok(Op::ReadWrite(ReadWriteOp::Pop(key)));
}

// -- AUTH --
fn parse_auth(parts: Vec<Part>) -> Result<Instruction, &'static str> {
    let auth: String;

    if parts.len() != 2 {
        return Err("AUTH requires 1 Argument: <Auth>")
    }

    load_or_err!(auth, match_into_value(parts.get(1)));

    Ok(Instruction::Authenticate(auth))
}