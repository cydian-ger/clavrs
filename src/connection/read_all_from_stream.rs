use std::{net::TcpStream, io::{Error, ErrorKind, Read}};

use super::connection::{BUFFER_SIZE, READ_TIME_OUT, END_OF_MESSAGE, MAX_MESSAGE_LENGTH, MAX_SOCKET_BUFFER_LENGTH};
#[derive(Debug)]
pub enum TcpError {
    MessageTooLong(usize),
    MessageExceedsMaxLength(),
    MessagePolluted(String),
    MessageUtf8Error(),
    TcpTimeout(),
    TcpShutdown(),
    TcpShutdownFromClient(),
    TcpUnkownError(Error),
}


pub fn read_all_from_stream(mut stream: &TcpStream) -> Result<String, TcpError> {
    let mut buffer = vec![0; BUFFER_SIZE];
    let mut message = String::new();
    let mut bytes_read;
    let mut initial_read: bool = true;

    loop {
        match stream.read(&mut buffer) {
            Ok(n) => bytes_read = n,
            Err(err) => {
                match err.kind() {
                    ErrorKind::ConnectionReset => {
                        return Err(TcpError::TcpShutdownFromClient());
                    }
                    ErrorKind::TimedOut => {
                        return Err(TcpError::TcpTimeout());
                    }
                    _ => {
                        return Err(TcpError::TcpUnkownError(err));
                    }
                }
                // distinguish errors
            }
        }

        // add timeout after the initial read
        if initial_read {
            let _ = stream.set_read_timeout(Some(READ_TIME_OUT));
        }
        initial_read = false;

        match String::from_utf8(buffer) {
            Ok(string) => {
                message += &string[..bytes_read];
            }
            Err(_err) => {
                return Err(TcpError::MessageUtf8Error());
            }
        }
        // message += &String::from_utf8(buffer[..bytes_read]).unwrap();

        // Problem when exactly 1024 characters are sent
        if message.contains(END_OF_MESSAGE) {
            break;
        }

        if message.len() == 0 {
            return Err(TcpError::TcpShutdown());
        }

        if message.len() > MAX_MESSAGE_LENGTH {
            // set time out for if exactly 3072 bytes are being sent.

            // read out remaining bytes
            let bytes: usize;
            let mut peek_buf: [u8; MAX_SOCKET_BUFFER_LENGTH] = [0; MAX_SOCKET_BUFFER_LENGTH];
            match stream.peek(&mut peek_buf) {
                Ok(u) => {
                    bytes = u;

                    // Return if it exceeds max buffer length
                    if u > (MAX_SOCKET_BUFFER_LENGTH - (MAX_MESSAGE_LENGTH + BUFFER_SIZE)) {
                        return Err(TcpError::MessageExceedsMaxLength());
                    }

                    // Clear out buffer
                    let mut temp_buf: [u8; 1] = [0; 1];
                    for _ in 1..u {
                        let _ = stream.read_exact(&mut temp_buf);
                    }
                }
                Err(err) => match err.kind() {
                    ErrorKind::ConnectionReset => {
                        return Err(TcpError::TcpShutdownFromClient());
                    }
                    ErrorKind::TimedOut => {
                        return Err(TcpError::TcpTimeout());
                    }
                    _ => {
                        return Err(TcpError::TcpUnkownError(err));
                    }
                },
            }
            return Err(TcpError::MessageTooLong(
                bytes + MAX_MESSAGE_LENGTH + BUFFER_SIZE,
            ));
        }

        buffer = vec![0; 1024];
    }

    if message.chars().filter(|x| *x == END_OF_MESSAGE).count() > 1 {
        // TODO
        // give back an error for polluted data.
        if let Some(index) = message.find(END_OF_MESSAGE) {
            return Err(TcpError::MessagePolluted(
                message[index..].to_string().replace(END_OF_MESSAGE, ""),
            ));
        } else {
            return Err(TcpError::MessagePolluted(String::from("")));
        }
    }

    message = message.replace(END_OF_MESSAGE, "");

    Ok(message)
}
