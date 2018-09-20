extern crate hostname;
extern crate bufstream;

use std::error::Error;
use std::net::{TcpStream};
use std::io;
use std::io::{Write,BufRead,stderr};
use bufstream::BufStream;

pub fn main() {
    println!("hello, here is receiver.");
}

pub fn handler(stream: &mut BufStream<TcpStream>) -> io::Result<()> {
    stream.write(b"220 hostname ESMTP smtpd-rs\n")?;
    stream.flush()?;

    loop {
        let mut first_line = String::new();
        if let Err(err) = stream.read_line(&mut first_line) {
            print_error(&err);
            panic!("error: {}", err);
        }

        let _message = dispatch(first_line)?;
    }
}

fn dispatch(command: String) -> io::Result<String> {
    let mut iter = command.split_whitespace();
    let method = iter.next();
    
    match method {
        Some("HELO") => {
            let peer_hostname = iter.next();
            println!("{:?}", peer_hostname);
            let reply = format!(r"250 ok at your service");
            Ok(reply)
        }
        Some("EHLO") => {
            let peer_hostname = iter.next();
            println!("{:?}", peer_hostname);
            let reply = format!(r"250 ok at your service");
            Ok(reply)
        }
        Some("MAIL FROM:") => {
            let from_address = iter.next();
            println!("{:?}", from_address);
            Ok(r"250 2.1.0 OK - smtpd-rs".parse().unwrap())
        }
        Some("RCPT TO:") => {
            let rcpt_address = iter.next();
            println!("{:?}", rcpt_address);
            Ok(r"250 2.1.0 OK - smtpd-rs".parse().unwrap())
        }
        Some("DATA") => {
            Ok(r"250 2.1.0 OK - smtpd-rs".parse().unwrap())
        }
        Some(".") => {
            Ok(r"250 2.1.0 OK - smtpd-rs".parse().unwrap())
        }
        Some("RSET") => {
            Ok(r"250 2.1.0 OK - smtpd-rs".parse().unwrap())
        }
        Some("QUIT") => {
            Ok(r"221 2.0.0 closing connection smtpd-rs".parse().unwrap())
        }
        _ => {
            println!("error");
            Ok(r"555 5.5.2 Syntax error. - smtpd-rs".parse().unwrap())
        }
    }
}

fn print_error(mut err: &Error) {
    let _ = writeln!(stderr(), "error: {}", err);
    while let Some(cause) = err.cause() {
        let _ = writeln!(stderr(), "caused by: {}", cause);
        err = cause;
    }
}

#[test]
fn test_handler() {
}

#[test]
fn test_dispatch() {
    assert_eq!(dispatch("QUIT".parse().unwrap()).ok(), Some(r"221 2.0.0 closing connection smtpd-rs".parse().unwrap()));
    assert_eq!(dispatch("HELO localhost".parse().unwrap()).ok(), Some(r"250 ok at your service".parse().unwrap()));
    assert_eq!(dispatch("EHLO localhost".parse().unwrap()).ok(), Some(r"250 ok at your service".parse().unwrap()));
    // assert_eq!(dispatch("MAIL FROM:postmaster@example.com".parse().unwrap()).ok(), Some(r"250 2.1.0 OK - smtpd-rs".parse().unwrap()));
    // assert_eq!(dispatch("RCPT TO:postmaster@example.com".parse().unwrap()).ok(), Some(r"250 2.1.0 OK - smtpd-rs".parse().unwrap()));
    assert_eq!(dispatch("DATA".parse().unwrap()).ok(), Some(r"250 2.1.0 OK - smtpd-rs".parse().unwrap()));
    assert_eq!(dispatch(".".parse().unwrap()).ok(), Some(r"250 2.1.0 OK - smtpd-rs".parse().unwrap()));
    assert_eq!(dispatch("RSET".parse().unwrap()).ok(), Some(r"250 2.1.0 OK - smtpd-rs".parse().unwrap()));
}
