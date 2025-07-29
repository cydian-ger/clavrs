#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    ILLEGAL(char),
    IDENT(Vec<char>),
    VALUE(Vec<char>),
    LIFETIME(Vec<char>),
    DELIMITER(char),
    KEYWORD(Keyword, KeywordType),
    // INSTRUCTION(Instruction),
    LIFETIMEREFERENCE,
    SPACE,
    LINEBREAK,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
}

#[derive(PartialEq, Debug, Clone)]
pub enum KeywordType {
    Operation, // Get Read etc
    Instruction, // Transactions
}

#[derive(PartialEq, Debug, Clone)]
pub enum Keyword {
    // Tokens
    // Read
    GET,
    EXISTS,
    HAS,
    // Write
    PUT,
    DELETE,
    CLEAR,
    REPLACE,
    RETRACT,
    // Restricted-Write
    PURGE,
    // Read Write
    POP,
    REDUCE,
    // Transaction
    SEQEUENCE,
    ABORT,
    EXECUTE,
}

pub fn get_keyword_token(ident: &Vec<char>) -> Result<Token, String> {
    match match_keyword(ident) {
        Ok(keyword) => { return Ok(Token::KEYWORD(keyword, KeywordType::Operation)) },
        Err(_) => {}
    }

    match match_instruction(ident) {
        Ok(keyword) => {return Ok(Token::KEYWORD(keyword, KeywordType::Instruction))},
        Err(_) => {}
    }

    return Err(String::from("Not a keyword"));
}

fn match_keyword(ident: &Vec<char>) -> Result<Keyword, ()> {
    let identifier: String = ident.into_iter().collect();
    match &identifier.to_lowercase()[..] {
        // Read
        "get" => Ok(Keyword::GET),
        "exists" => Ok(Keyword::EXISTS),
        "has" => Ok(Keyword::HAS),
        // Write
        "put" => Ok(Keyword::PUT),
        "delete" => Ok(Keyword::DELETE),
        "clear" => Ok(Keyword::CLEAR),
        "replace" => Ok(Keyword::REPLACE),
        "retract" => Ok(Keyword::RETRACT),
        // Restricted
        "purge" => Ok(Keyword::PURGE),
        // Read Write
        "pop" => Ok(Keyword::POP),
        "reduce" => Ok(Keyword::REDUCE),
        _ => Err(())
    }
}

fn match_instruction(ident: &Vec<char>) -> Result<Keyword, ()> {
    let identifier: String = ident.into_iter().collect();
    match &identifier.to_lowercase()[..] {
        // Instruction
        // Transactions
        "sequence" => Ok(Keyword::SEQEUENCE),
        "abort" => Ok(Keyword::ABORT),
        "execute" => Ok(Keyword::EXECUTE),
        _ => {Err(())}
    }
}
