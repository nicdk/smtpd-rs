extern crate hostname;
extern crate bufstream;

use std::error::Error;
use std::thread::spawn;
use std::io::{Write,BufRead,stderr};
use std::net::{TcpListener,TcpStream};
use bufstream::BufStream;
use std::sync::mpsc;
use std::sync::mpsc::{Sender,Receiver};

fn main() {
    println!("Hello, world!");

    let listener = TcpListener::bind("127.0.0.1:8025").unwrap();
    let (_, recv): (Sender<String>, Receiver<String>) = mpsc::channel();
    spawn(move || loop {
        let msg = recv.recv().unwrap();
        print!("DEBUG: msg {}", msg);
    });
          
    for stream in listener.incoming() {
        match stream {
            Err(_) => panic!("listen error"),
            Ok(mut stream) => {
                println!("connection from {} to {}", stream.peer_addr().unwrap(), stream.local_addr().unwrap());
                spawn(move || {
                    let mut stream = BufStream::new(stream);
                    handle_client(&mut stream)
                });
            }
        }
    }
}

fn handle_client(stream: &mut BufStream<TcpStream>) {
    stream.write(b"220 hostname ESMTP smtpd-rs\n").unwrap();
    stream.flush().unwrap();
    
    let mut first_line = String::new();
    if let Err(err) = stream.read_line(&mut first_line) {
        print_error(&err);
        panic!("error: {}", err);
    }

    dispatch(first_line).unwrap()
}

fn dispatch(command: String) -> Result<(), ()> {
    let mut iter = command.split_whitespace();
    let method = iter.next();
    
    match method {
        Some("HELO") => {
            let peer_hostname = iter.next();
            println!("{:?}", peer_hostname);
            ()
        }
        Some("EHLO") => {
            let peer_hostname = iter.next();
            println!("{:?}", peer_hostname);
            ()
        }
        Some("MAIL FROM:") => {
            let from_address = iter.next();
            println!("{:?}", from_address);
            ()
        }
        Some("RCPT TO:") => {
            let rcpt_address = iter.next();
            println!("{:?}", rcpt_address);
            ()
        }
        Some("DATA") => {
            ()
        }
        Some(".") => {
            ()
        }
        Some("RSET") => {
            ()
        }
        Some("QUIT") => {
            println!("QUIT");
            print!(r"221 2.0.0 closing connection smtpd-rs");
            ()
        }
        _ => {
            println!("error");
            ()
        }
    }
    Ok(())
}

fn print_error(mut err: &Error) {
    let _ = writeln!(stderr(), "error: {}", err);
    while let Some(cause) = err.cause() {
        let _ = writeln!(stderr(), "caused by: {}", cause);
        err = cause;
    }
}

#[test]
fn test_dispatch() {
    assert_eq!(dispatch(String::from("QUIT")), Ok(()));
    assert_eq!(dispatch(String::from("HELO localhost")), Ok(()));
    assert_eq!(dispatch(String::from("EHLO localhost")), Ok(()));
    assert_eq!(dispatch(String::from("MAIL FROM:postmaster@example.com")), Ok(()));
    assert_eq!(dispatch(String::from("RCPT TO:postmaster@example.com")), Ok(()));
    assert_eq!(dispatch(String::from("DATA")), Ok(()));
    assert_eq!(dispatch(String::from(".")), Ok(()));
    assert_eq!(dispatch(String::from("RSET")), Ok(()));
}
