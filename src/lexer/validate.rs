// use chrono::{DateTime, Local};

use super::token::{Keyword, Token, KeywordType};

pub fn is_valid(tokens: Vec<Token>) -> Result<Vec<Part>, String> {
    let mut b = Builder::new();
    for (index, token) in tokens.into_iter().enumerate() {
        match b.transition(token) {
            Ok(_) => {}
            Err(err) => {
                return Err(format!("{}: {}", index + 1, err));
            }
        }
    }
    Ok(b.part_buffer)
}

#[derive(Debug)]
pub enum Lifetime {
    Static,
    // Date(DateTime<Local>),
    User(String),
    Connection(String)
}

impl Lifetime {
    pub fn from_string(lt: String, value: Option<String>) -> Result<Lifetime, &'static str> {
        match lt.as_str() {
            "'s" => {
                if value.is_some() { return Err("Static Lifetime does not take Value")}
                return Ok(Lifetime::Static)
            }
            "'d" => { todo!() }
            "'u" => {
                match value {
                    Some(user_hash) => { return Ok(Lifetime::User(user_hash))}
                    None => { return Err("User Lifetime needs User Hash as Value") }
                }
            }
            "'c" => {
                match value {
                    Some(connection_hash) => { return Ok(Lifetime::Connection(connection_hash))}
                    None => { return Err("Connection Lifetime needs Connection Hash as Value")}
                }
            }
            _ => {return Err("")}
        }
    }
}

#[derive(Debug)]
pub enum Part {
    Keyword {
        keyword: Keyword,
        keyword_type: KeywordType,
    },
    Lifetime {
        reference_name: Option<String>,
        lifetime: Lifetime,
    },
    Value {
        value: String
    },
    Values {
        values: Vec<String>,
    },
    NestedValues {
        values: Vec<Vec<String>>,
    },
}

#[derive(Debug)]
enum State {
    DEFAULT,
    VALUES,
    VALUE,
    DELIMITER,
    NESTEDNEXT,
    NESTEDNEXTDEL,
    PRELIFETIME,
    LIFETIME,
    FILLEDLIFETIME,
    NAMEDLIFETIME,
}

#[derive(Debug)]
enum Context {
    DEFAULT,
    VALUE,
    NESTEDVALUE,
    REFERENCE,
}

struct Builder<S, C> {
    state: S,
    context: C,
    values_buffer: Vec<String>,
    nested_values_buffer: Vec<Vec<String>>,
    lifetime_reference_name_buffer: Option<String>,
    lifetime_name_buffer: Option<String>,
    lifetime_value_buffer: Option<String>,
    part_buffer: Vec<Part>,
}

impl Builder<State, Context> {
    pub fn new() -> Builder<State, Context> {
        Builder {
            state: State::DEFAULT,
            context: Context::DEFAULT,
            values_buffer: vec![],
            nested_values_buffer: vec![],
            lifetime_reference_name_buffer: None,
            lifetime_name_buffer: None,
            lifetime_value_buffer: None,
            part_buffer: vec![],
        }
    }

    fn reset(&mut self) {
        self.state = State::DEFAULT;
        self.context = Context::DEFAULT;
        self.values_buffer.clear();
        self.nested_values_buffer.clear();
    }

    fn construct_lifetime(&mut self) -> Result<(), &'static str> {
        let name: String;
        let lt: Lifetime;
        match &self.lifetime_name_buffer {
            None => {return Err("Lifetime created without Lifetime")}
            Some(lt_name) => { name = lt_name.clone()}
        }
        match Lifetime::from_string(name, self.lifetime_value_buffer.clone()) {
            Ok(lifetime) => { lt = lifetime },
            Err(err) => { return Err(err) }
        }
        self.part_buffer.push(Part::Lifetime { reference_name: self.lifetime_reference_name_buffer.clone(), lifetime: lt });
        Ok(())
    }

    fn transition(&mut self, next_token: Token) -> Result<(), String> {
        match (&self.state, &self.context) {
            (State::DEFAULT, Context::DEFAULT) => {
                match next_token {
                    Token::LPAREN => {
                        self.state = State::VALUES;
                        self.context = Context::VALUE
                    }
                    Token::KEYWORD(kw, kw_type) => {
                        self.part_buffer.push(Part::Keyword { keyword: kw , keyword_type: kw_type})
                        // Do nothing
                    }
                    Token::LBRACE => {
                        self.state = State::PRELIFETIME;
                    }
                    Token::VALUE(val) => {
                        self.part_buffer.push(Part::Value { value: val.iter().collect() })
                    }
                    err_token => return Err(format!("Invalid Token {:?} after Default state", err_token)),
                }
                Ok(())
            }

            // STATE FOR VALUES
            (State::VALUES, context) => {
                match next_token {
                    Token::VALUE(v) => {
                        let value: String = v.iter().collect();
                        self.state = State::VALUE;

                        match context {
                            Context::VALUE => self.values_buffer.push(value),
                            Context::NESTEDVALUE => {
                                self.nested_values_buffer.last_mut().unwrap().push(value)
                            }
                            _ => return Err("Invalid Context after Value in Values".to_string()),
                        }
                    }
                    Token::LPAREN => {
                        match context {
                            Context::VALUE => {
                                self.context = Context::NESTEDVALUE;
                                self.nested_values_buffer.push(vec![])
                            }
                            _ => return Err(
                                "Invalid Context for Values to receive another left parenthesis".to_string(),
                            ),
                        }
                    }
                    err_token => return Err(format!("Invalid Token {:?} after Values", err_token)),
                }

                Ok(())
            }
            (State::VALUE, context) => {
                match next_token {
                    Token::DELIMITER(_) => {
                        self.state = State::DELIMITER;
                    }
                    Token::RPAREN => match context {
                        Context::VALUE => {
                            self.part_buffer.push(Part::Values {
                                values: self.values_buffer.iter().map(|x| x.clone()).collect(),
                            });
                            self.reset();
                        }
                        Context::NESTEDVALUE => self.state = State::NESTEDNEXT,
                        _ => return Err("Invalid context for right parenthesis in Value.".to_string()),
                    },
                    err_token => return Err(format!("Invalid Token {:?} after Value", err_token)),
                };
                Ok(())
            }
            (State::DELIMITER, context) => {
                match next_token {
                    Token::VALUE(v) => {
                        self.state = State::VALUE;
                        let value: String = v.iter().collect();

                        match context {
                            Context::NESTEDVALUE => {
                                self.nested_values_buffer.last_mut().unwrap().push(value);
                            }
                            Context::VALUE => {
                                self.values_buffer.push(value);
                            }
                            _ => {
                                return Err("Invalid context for Delimiter".to_string());
                            }
                        }
                    }
                    err_token => return Err(format!("Invalid Token {:?} after Delimited", err_token)),
                }

                Ok(())
            }
            (State::NESTEDNEXT, Context::NESTEDVALUE) => {
                match next_token {
                    Token::RPAREN => {
                        self.part_buffer.push(Part::NestedValues {
                            values: self
                                .nested_values_buffer
                                .iter()
                                .map(|x| x.clone())
                                .collect(),
                        });
                        self.reset();
                    }
                    Token::DELIMITER(_) => {
                        self.nested_values_buffer.push(Vec::new());
                        self.state = State::NESTEDNEXTDEL;
                    }
                    err_token => return Err(format!("Invalid Token {:?} after Nested Next.", err_token)),
                }
                Ok(())
            }
            (State::NESTEDNEXTDEL, Context::NESTEDVALUE) => {
                match next_token {
                    Token::LPAREN => self.state = State::VALUES,
                    err_token => return Err(format!("Invalid Token {:?} after Nested Next Delimiter.", err_token)),
                }

                Ok(())
            }

            // STATE FOR LIFETIME
            (State::PRELIFETIME, c) => {
                match next_token {
                    Token::LIFETIMEREFERENCE => match c {
                        Context::REFERENCE => return Err("Reference appeared twice".to_string()),
                        _ => self.context = Context::REFERENCE,
                    },
                    Token::LIFETIME(lt) => {
                        self.lifetime_name_buffer = Some(lt.iter().collect());
                        self.state = State::LIFETIME;
                    }
                    err_token => return Err(format!("Invalid Token {:?} after Pre Lifetime", err_token)),
                }
                Ok(())
            }

            (State::LIFETIME, c) => {
                match next_token {
                    Token::VALUE(v) => {
                        self.state = State::FILLEDLIFETIME;
                        self.lifetime_value_buffer = Some(v.iter().collect());
                    }
                    Token::RBRACE => match c {
                        Context::DEFAULT => {
                            match self.construct_lifetime() {
                                Ok(_) => {},
                                Err(err) => {return Err(err.to_string())}
                            }
                            self.reset();
                        }
                        _ => { return Err("Tried to exit Referenced Lifetime before giving Ident of Reference".to_string())}
                    },
                    Token::IDENT(ident) => {
                        self.lifetime_reference_name_buffer = Some(ident.iter().collect());
                        self.state = State::NAMEDLIFETIME;
                    }
                    err_token => return Err(format!("Invalid Token {:?} after Lifetime", err_token)),
                }
                Ok(())
            }

            (State::FILLEDLIFETIME, c) => {
                match next_token {
                    Token::IDENT(ident) => {
                        self.lifetime_reference_name_buffer = Some(ident.iter().collect());
                        self.state = State::NAMEDLIFETIME;
                    }
                    Token::RBRACE => match c {
                        Context::DEFAULT => {
                            match self.construct_lifetime() {
                                Ok(_) => {},
                                Err(err) => {return Err(err.to_string())}
                            }
                            self.reset();
                        }
                        _ => { return Err("Tried to exit Referenced Lifetime before giving Ident of Reference".to_string())}
                    }
                    err_token => { return Err(format!("Invalid Token {:?} after Filled Lifetime", err_token))}
                }
                Ok(())
            }

            (State::NAMEDLIFETIME, _) => {
                match next_token {
                    Token::RBRACE => {
                        match self.construct_lifetime() {
                            Ok(_) => {},
                            Err(err) => {return Err(err.to_string())}
                        }
                        self.reset();
                    }
                    err_token => {return Err(format!("Invalid Token {:?} after Named Lifetime", err_token));}
                }
                Ok(())
            } 
            (s, c) => return Err(format!("Invalid State {:?} with context {:?}", s, c)),
        }
    }
}
