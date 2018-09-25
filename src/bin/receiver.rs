extern crate hostname;
extern crate bufstream;

use std::error::Error;
use std::net::{TcpStream};
use std::io;
use std::io::{Write,BufRead,stderr};
use bufstream::BufStream;

#[derive(Debug, PartialEq)]
struct SmtpReply {
    pub code: u16,
    pub message: String
}

pub fn main() {
    println!("hello, here is receiver.");
}

pub fn handler(stream: &mut BufStream<TcpStream>) -> io::Result<()> {
    stream.write(b"220 hostname ESMTP smtpd-rs\n")?;
    stream.flush()?;

    loop {
        let mut line = String::new();
        if let Err(err) = stream.read_line(&mut line) {
            print_error(&err);
            panic!("error: {}", err);
        }

        match dispatch(line) {
            Some(_reply) => {
                let _code = &_reply.code;
                let _message: &str = &_reply.message;
                stream.write(&_message.as_bytes())?;
                stream.flush()?;
            }
            None => {
                stream.write(b"disconnect.")?;
                stream.flush()?;
                panic!();
            }
        }
    }
}

fn dispatch(command: String) -> Option<SmtpReply> {
    let mut iter = command.split_whitespace();
    let _method = iter.next();
    
    match _method {
        Some("HELO") => {
            let param1 = iter.next();
            println!("param1 {:?}", param1);
            let reply;
            match param1 {
                None => {
                    reply = SmtpReply{
                        code: 555,
                        message: format!(r"5.5.2 Syntax error. - smtpd-rs")
                    }
                }
                Some(param1) => {
                    if iter.next() != None {
                        reply = SmtpReply{
                            code: 555,
                            message: format!(r"5.5.2 Syntax error. - smtpd-rs")
                        };
                    } else {
                        reply = SmtpReply{
                            code: 250,
                            message: format!(r"ok at your service {:?}", param1)
                        };
                    }
                }
            }
            Some(reply)
        }
        Some("EHLO") => {
            let peer_hostname = iter.next();
            println!("{:?}", peer_hostname);
            let reply = SmtpReply{
                code: 250,
                message: format!(r"ok at your service")
            };
            Some(reply)
        }
        Some("MAIL") => {
            let from_address = iter.next();
            println!("{:?}", from_address);
            let reply = SmtpReply{
                code: 250,
                message: format!(r"ok at your service")
            };
            Some(reply)
        }
        Some("RCPT") => {
            let rcpt_address = iter.next();
            println!("{:?}", rcpt_address);
            let reply = SmtpReply{
                code: 250,
                message: format!(r"ok at your service")
            };
            Some(reply)
        }
        Some("DATA") => {
            let reply = SmtpReply{
                code: 250,
                message: format!(r"2.1.0 OK - smtpd-rs")
            };
            Some(reply)
        }
        Some(".") => {
            let reply = SmtpReply{
                code: 250,
                message: format!(r"2.1.0 OK - smtpd-rs")
            };
            Some(reply)
        }
        Some("RSET") => {
            let reply = SmtpReply{
                code: 250,
                message: format!(r"2.1.0 OK - smtpd-rs")
            };
            Some(reply)
        }
        Some("QUIT") => {
            let reply = SmtpReply{
                code: 221,
                message: format!(r"2.0.0 closing connection smtpd-rs")
            };
            Some(reply)
        }
        _ => {
            println!("error");
            let reply = SmtpReply{
                code: 555,
                message: format!(r"5.5.2 Syntax error. - smtpd-rs")
            };
            Some(reply)
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
    assert_eq!(dispatch("QUIT".parse().unwrap()),           Some(SmtpReply{code: 221, message: r"2.0.0 closing connection smtpd-rs".to_string()}));
    assert_eq!(dispatch("HELO localhost".parse().unwrap()), Some(SmtpReply{code: 250, message: "ok at your service \"localhost\"".parse().unwrap()}));
    assert_eq!(dispatch("EHLO localhost".parse().unwrap()), Some(SmtpReply{code: 250, message: r"ok at your service".parse().unwrap()}));
    // assert_eq!(dispatch("MAIL FROM:postmaster@example.com".parse().unwrap()), Some(r"250 2.1.0 OK - smtpd-rs".parse().unwrap()));
    // assert_eq!(dispatch("RCPT TO:postmaster@example.com".parse().unwrap()), Some(r"250 2.1.0 OK - smtpd-rs".parse().unwrap()));
    assert_eq!(dispatch("DATA".parse().unwrap()), Some(SmtpReply{code: 250, message: r"2.1.0 OK - smtpd-rs".parse().unwrap()}));
    assert_eq!(dispatch(".".parse().unwrap()),    Some(SmtpReply{code: 250, message: r"2.1.0 OK - smtpd-rs".parse().unwrap()}));
    assert_eq!(dispatch("RSET".parse().unwrap()), Some(SmtpReply{code: 250, message: r"2.1.0 OK - smtpd-rs".parse().unwrap()}));
}
