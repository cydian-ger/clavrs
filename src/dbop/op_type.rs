#[derive(Debug)]
pub enum Op {
    Read(ReadOp),
    Write(WriteOp),
    ReadWrite(ReadWriteOp),
}

#[derive(Debug)]
pub enum WriteOp {
    Put(Vec<String>, Vec<Vec<String>>), // PUT [KEYS] [[VALUES]] -> puts the value list for every key

    // Delete types
    Delete(Vec<String>), // DELETE [KEYS] -> delete all keys + values
    Clear(Vec<String>),  // CLEAR [KEYS] -> clear values for keys

    // Update
    Replace(String, Vec<String>), // REPLACE KEY [VALUES]

    // Morph
    Retract(Vec<String>, Vec<String>), // RETRACT [KEYS] [VALUES_TO_RETRACT] -> retracts values if they exists from keys

    // Restricted
    Purge,
}

// CRUD [C = Put, R = Get, U = Replace, D = Delete]

#[derive(Debug)]
pub enum ReadWriteOp {
    // Delete / Delete
    Pop(String), // POP [KEY] -> [POP_VALUE]

    // Morph
    // Reduce(Vec<String>, Regex), // REDUCE [KEY] -> [REMOVED_VALUES]
}

#[derive(Debug)]
pub enum ReadOp {
    // Read
    Get(Vec<String>),         // GET [KEYS] -> [[VALUES]]
    Exists(Vec<String>),      // EXISTS [KEYS] -> [BOOL]
    Has(Vec<String>, String), // HAS [KEYS] VALUE -> [BOOL], Maybe make has take multiple values and give back [[BOOL]]
}

pub enum Instruction {
    Transaction(TransactionOp),
    Authenticate(String),
}

#[derive(Debug)]
pub enum TransactionOp {
    Sequence,
    Abort,
    Execute,
}
