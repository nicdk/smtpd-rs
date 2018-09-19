extern crate hostname;
extern crate bufstream;

use std::error::Error;
use std::net::{TcpListener,TcpStream};
use std::io;
use std::io::{Write,BufRead,stderr};
use std::thread::spawn;
use bufstream::BufStream;

fn main() {
    smtpd_main("127.0.0.1:8025").expect("error: ");
}

fn smtpd_main(addr: &str) -> io::Result<()> {
    let listener = TcpListener::bind(addr)?;
    println!("smtpd-rs: listening on {}", addr);
    listener.set_ttl(100).expect("could not set TTL");
    
    loop {
        let (stream, c_addr) = listener.accept()?;
        println!("connection received from {}", c_addr);

        spawn(move || {
            let mut smtp_stream = BufStream::new(stream);
            loop {
                handle_client(&mut smtp_stream).err();
            }
        });
    }
}

fn handle_client(stream: &mut BufStream<TcpStream>) -> io::Result<()> {
    stream.write(b"220 hostname ESMTP smtpd-rs\n")?;
    stream.flush()?;
    
    let mut first_line = String::new();
    if let Err(err) = stream.read_line(&mut first_line) {
        print_error(&err);
        panic!("error: {}", err);
    }

    let _message = dispatch(first_line)?;
    stream.write(b"220 hostname ESMTP smtpd-rs\n").unwrap();
    Ok(())
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
            Ok(r"250 2.1.0 OK - smtpd-rs".to_string())
        }
        Some("RCPT TO:") => {
            let rcpt_address = iter.next();
            println!("{:?}", rcpt_address);
            Ok(r"250 2.1.0 OK - smtpd-rs".to_string())
        }
        Some("DATA") => {
            Ok(r"250 2.1.0 OK - smtpd-rs".to_string())
        }
        Some(".") => {
            Ok(r"250 2.1.0 OK - smtpd-rs".to_string())
        }
        Some("RSET") => {
            Ok(r"250 2.1.0 OK - smtpd-rs".to_string())
        }
        Some("QUIT") => {
            Ok(r"221 2.0.0 closing connection smtpd-rs".to_string())
        }
        _ => {
            println!("error");
            Ok(r"555 5.5.2 Syntax error. - smtpd-rs".to_string())
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
fn test_handle_client() {
}

#[test]
fn test_dispatch() {
    assert_eq!(dispatch("QUIT".to_string()).ok(), Some(r"221 2.0.0 closing connection smtpd-rs".to_string()));
    assert_eq!(dispatch("HELO localhost".to_string()).ok(), Some(r"250 ok at your service".to_string()));
    assert_eq!(dispatch("EHLO localhost".to_string()).ok(), Some(r"250 ok at your service".to_string()));
    // assert_eq!(dispatch("MAIL FROM:postmaster@example.com".to_string()).ok(), Some(r"250 2.1.0 OK - smtpd-rs".to_string()));
    // assert_eq!(dispatch("RCPT TO:postmaster@example.com".to_string()).ok(), Some(r"250 2.1.0 OK - smtpd-rs".to_string()));
    assert_eq!(dispatch("DATA".to_string()).ok(), Some(r"250 2.1.0 OK - smtpd-rs".to_string()));
    assert_eq!(dispatch(".".to_string()).ok(), Some(r"250 2.1.0 OK - smtpd-rs".to_string()));
    assert_eq!(dispatch("RSET".to_string()).ok(), Some(r"250 2.1.0 OK - smtpd-rs".to_string()));
}
