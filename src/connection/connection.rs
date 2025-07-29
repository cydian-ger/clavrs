use std::{
    io::Write,
    net::TcpStream,
    sync::{Arc, Mutex},
    time::Duration,
};

use evmap::{ReadHandle, WriteHandle};

use crate::{
    connection::{
        connection_state::ConnectionState,
        handle_instruction::handle_instruction,
        handle_operation::handle_operation,
        read_all_from_stream::{read_all_from_stream, TcpError},
    },
    lexer::{
        lex::lex,
        token::KeywordType,
        validate::{is_valid, Part},
    },
};

use super::permission::Permission;

pub const END_OF_MESSAGE: char = ''; // use end of text
pub const BUFFER_SIZE: usize = 1024;
pub const MAX_MESSAGE_LENGTH: usize = 2048;
pub const MAX_SOCKET_BUFFER_LENGTH: usize = 8192;
pub const ERR_PREFIX: &str = "Err: ";

// Timeout time for read after first chunk of data was sent
pub const READ_TIME_OUT: Duration = Duration::new(10, 0);

// Timeout time since last full message sent
pub const IDLE_TIME_OUT: Option<Duration> = Some(Duration::new(20, 0));

pub fn handle_connection(
    mut stream: TcpStream,
    read_handle: ReadHandle<String, String>,
    write_mutex: Arc<Mutex<WriteHandle<String, String>>>,
    permission: Permission,
) {
    let mut connection_state = ConnectionState::new(read_handle, write_mutex);

    loop {
        // Continue means a recoverable error was transmitted
        // Return means an irrecoverable error was transmitted
        let message: String;

        match read_all_from_stream(&mut stream) {
            Ok(msg) => {
                // Reset the read timeout for the socket
                let _ = stream.set_read_timeout(IDLE_TIME_OUT);
                message = msg;
            }
            Err(err) => {
                let _ = stream.set_read_timeout(IDLE_TIME_OUT);

                // Continue if the error is recoverable else shutdown the Connection
                match &err {
                    // Message exceeds MAX_MESSAGE_LENGTH but is not considered as spam / overloading the db
                    TcpError::MessageTooLong(bytes) => {
                        let _ = stream.write(
                            format!(
                                "{}Message was longer than allowed {} bytes, ({})",
                                ERR_PREFIX, MAX_MESSAGE_LENGTH, bytes
                            )
                            .as_bytes(),
                        );
                        continue;
                    }

                    // Message exceeds the max buffer length and is seen as deliberate spam, thus connection is closed
                    TcpError::MessageExceedsMaxLength() => {
                        let _ = stream.write(
                            format!(
                                "{}Message exceeded max length {}, Connection closed.",
                                ERR_PREFIX, MAX_SOCKET_BUFFER_LENGTH
                            )
                            .as_bytes(),
                        );
                    }

                    // Message has multiple end_of_message bytes
                    TcpError::MessagePolluted(_pollution) => {
                        //
                        let _ = stream.write(
                            format!(
                                "{}Message is Polluted. Polluted Data: '{}'",
                                ERR_PREFIX, _pollution
                            )
                            .as_bytes(),
                        );
                        continue;
                    }

                    // Invalid Utf8 for message
                    TcpError::MessageUtf8Error() => {
                        let _ = stream.write(
                            format!("{}Invalid Utf8, Connection closed.", ERR_PREFIX).as_bytes(),
                        );
                    }
                    //
                    TcpError::TcpShutdown() => {}
                    TcpError::TcpShutdownFromClient() => {}

                    // Tcp Socket Timed out
                    TcpError::TcpTimeout() => {
                        // Maybe log the ip for it
                        let _ = stream.write(
                            format!(
                                "{}Connection timed out- Connection closed. Read_timeout:{:?}, Idle_timeout:{:#?}",
                                ERR_PREFIX, READ_TIME_OUT, IDLE_TIME_OUT
                            )
                            .as_bytes(),
                        );
                    }

                    // Every other error
                    TcpError::TcpUnkownError(_string) => {}
                }

                // Irrecoverable Error
                println!(
                    "{:?}: Closing connection due to {:?}",
                    stream.peer_addr().unwrap(),
                    err
                );
                let _ = stream.shutdown(std::net::Shutdown::Both);
                return;
            }
        }

        //
        let _ = stream.set_read_timeout(IDLE_TIME_OUT);

        if message == String::from("QUIT") {
            break;
        }

        // Lex the input string into tokens
        let tokens = lex(message);

        // Check through StateMachine if the given tokens are in a valid construct
        // If they are it loads them into parts and continues
        let parts: Vec<Part>;

        match is_valid(tokens) {
            Ok(validated_parts) => {
                parts = validated_parts;
            }
            Err(err) => {
                if err.len() == 0 {
                    let _ = stream.write(format!("{}Unidentified Error", ERR_PREFIX).as_bytes());
                } else {
                    let _ = stream.write(format!("{}{}", ERR_PREFIX, err).as_bytes());
                }
                continue;
            }
        }

        if parts.len() == 0 {
            let _ = stream.write(format!("{} Empty Command", ERR_PREFIX).as_bytes());
        }
        
        // Check permissions

        // Have meta operations here too
        // Operation vs Instruction
        match parts.first().unwrap() {
            Part::Keyword {
                keyword: _,
                keyword_type,
            } => {
                match keyword_type {
                    KeywordType::Operation => {
                        match handle_operation(parts, &mut connection_state, &permission) {
                            Ok(ok) => {
                                let _ = stream.write(ok.as_bytes());
                            }
                            Err(err) => {
                                let _ = stream.write(format!("{}{}", ERR_PREFIX, err).as_bytes());
                                continue;
                            }
                        }
                    }
                    KeywordType::Instruction => {
                        match handle_instruction(parts, &mut connection_state, &permission) {
                            Ok(ok) => {
                                let _ = stream.write(ok.as_bytes());
                            }
                            Err(err) => {
                                let _ = stream.write(format!("{}{}", ERR_PREFIX, err).as_bytes());
                                continue;
                            }
                        }
                    }
                };
            }

            _ => {
                let _ = stream
                    .write(format!("{}This Error should never be thrown. But fuck it lets have it here anyway.", ERR_PREFIX).as_bytes());
                continue;
            }
        }
    }

    // This is only for correct termination
    println!("Closing connection: {:?}", stream.peer_addr().unwrap());
    let _ = stream.shutdown(std::net::Shutdown::Both);
}
