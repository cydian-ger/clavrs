mod connection;
mod dbop;
mod lexer;

use clap::{Parser, ValueEnum};
use connection::connection::handle_connection;
use evmap::ReadHandleFactory;
use std::{
    net::TcpListener,
    sync::{Arc, Mutex},
    thread,
};

use crate::connection::{permission::Permission, permission_list::{PermissionState, PermissionList}};

#[derive(ValueEnum, Debug, Clone)] // ArgEnum here
#[clap(rename_all = "kebab_case")]
enum Mode {
    Test,
    Default,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // Address under which the TcpListener will be bound
    #[arg(short, long, default_value_t = String::from("127.0.0.1:3254"))]
    address: String,

    #[arg(short, long, value_enum, default_value_t = Mode::Default)]
    mode: Mode,

    #[arg(short, long)]
    perm_path: Option<String>
}

fn main() {
    // Init the kernel section
    let (read, write) = evmap::new();
    let read_factory: ReadHandleFactory<String, String> = read.factory();
    let read_mutex = Arc::new(Mutex::new(write));

    let command_line_args = Args::parse();
    let listener = TcpListener::bind(&command_line_args.address).unwrap();
    println!(
        "Clavrs is running at {} in {:?}-Mode",
        &command_line_args.address, &command_line_args.mode
    );

    let permissions: PermissionList = PermissionList::from_path(command_line_args.perm_path);

    println!("{}", serde_json::to_string(&permissions).unwrap());

    for stream in listener.incoming() {

        // TODO
        // Permission could be checked here, and loaded on updated file

        match stream {
            Ok(stream) => {
                let read_handle = read_factory.handle();
                let write_mutex = Arc::clone(&read_mutex);

                println!("{:?}: Connection Established", stream.peer_addr().unwrap());

                let db_mode = command_line_args.mode.clone();
                thread::spawn(|| {
                    // TODO
                    // authenticate connection first.
                    // if auth fail just close connection
                    handle_connection(
                        stream,
                        read_handle,
                        write_mutex,
                        Permission::new(&PermissionState::default(), db_mode),
                    );
                });
            }
            Err(_) => {}
        }
    }
}
