extern crate hostname;
extern crate bufstream;

use std::error::Error;
use std::net::{TcpStream};
use std::io;
use std::io::{Write,BufRead,stderr};
use bufstream::BufStream;

enum SmtpState {
    INIT,
    READY,
    MAIL,
    RCPT,
    DATA,
}

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
    let mut state = SmtpState::INIT;

    loop {
        let mut line = String::new();
        if let Err(err) = stream.read_line(&mut line) {
            print_error(&err);
            panic!("error: {}", err);
        }

        let _cmd;
        match line.trim().split_whitespace().nth(0) {
            Some(s) => _cmd = s,
            None => {
                stream.write(b"555 5.5.2 Syntax error. - smtpd-rs")?;
                stream.flush()?;
                continue;
            }
        };
        println!("command {}", _cmd);

        match dispatch(&line) {
            Some(_reply) => {
                let _code = &_reply.code;
                let _message: &str = &_reply.message;
                stream.write(&_message.as_bytes())?;
                stream.flush()?;

                state = match &state {
                    SmtpState::INIT => {
                        match &_cmd {
                            &"HELO" => SmtpState::READY,
                            &"EHLO" => SmtpState::READY,
                            &"RSET" => SmtpState::READY,
                            _ => SmtpState::INIT,
                        }
                    }
                    SmtpState::READY => {
                        match &_cmd {
                            &"MAIL" => SmtpState::MAIL,
                            &"RSET" => SmtpState::READY,
                            _ => SmtpState::READY,
                        }
                    }
                    SmtpState::MAIL => {
                        match &_cmd {
                            &"RCPT" => SmtpState::RCPT,
                            &"RSET" => SmtpState::READY,
                            _ => SmtpState::MAIL,
                        }
                    }
                    SmtpState::RCPT => {
                        match &_cmd {
                            &"DATA" => SmtpState::DATA,
                            &"RSET" => SmtpState::READY,
                            _ => SmtpState::RCPT,
                        }
                    }
                    SmtpState::DATA => {
                        match &_cmd {
                            &"DATA" => SmtpState::READY,
                            &"RSET" => SmtpState::READY,
                            _ => SmtpState::DATA,
                        }
                    }
                }
            }
            None => {
                /* abort. */
                stream.write(b"disconnect.")?;
                stream.flush()?;
                panic!();
            }
        }
    }
}

fn dispatch(command: &String) -> Option<SmtpReply> {
    let mut command_split = command.trim().split_whitespace();
    let _cmd = command_split.next();

    /* minimum implementation (see RFC5321 4.5.1) */
    match _cmd {
        Some("EHLO") => {
            let param1 = command_split.next();
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
                    if command_split.next() != None {
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
        Some("HELO") => {
            let param1 = command_split.next();
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
                    if command_split.next() != None {
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
        Some("MAIL") => {
            let from_address = command_split.next();
            println!("{:?}", from_address);
            let reply = SmtpReply{
                code: 250,
                message: format!(r"2.1.0 OK - smtpd-rs")
            };
            Some(reply)
        }
        Some("RCPT") => {
            let rcpt_address = command_split.next();
            println!("{:?}", rcpt_address);
            let reply = SmtpReply{
                code: 250,
                message: format!(r"2.1.0 OK - smtpd-rs")
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
        Some("RSET") => {
            let reply = SmtpReply{
                code: 250,
                message: format!(r"2.1.0 OK - smtpd-rs")
            };
            Some(reply)
        }
        Some("NOOP") => {
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
        Some("VRFY") => {
            let reply = SmtpReply{
                code: 250,
                message: format!(r"2.1.0 OK - smtpd-rs")
            };
            Some(reply)
        }
        _ => {
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
    assert_eq!(dispatch(&"EHLO localhost".to_string()), Some(SmtpReply{code: 250, message: "ok at your service \"localhost\"".parse().unwrap()}));
    assert_eq!(dispatch(&"HELO localhost".to_string()), Some(SmtpReply{code: 250, message: "ok at your service \"localhost\"".parse().unwrap()}));

    assert_eq!(dispatch(&"MAIL FROM:postmaster@example.com".to_string()), Some(SmtpReply{code: 250, message: r"2.1.0 OK - smtpd-rs".parse().unwrap()}));
    assert_eq!(dispatch(&"RCPT TO:postmaster@example.com".to_string()),   Some(SmtpReply{code: 250, message: r"2.1.0 OK - smtpd-rs".parse().unwrap()}));
    assert_eq!(dispatch(&"DATA".to_string()), Some(SmtpReply{code: 250, message: r"2.1.0 OK - smtpd-rs".parse().unwrap()}));

    assert_eq!(dispatch(&"RSET".to_string()), Some(SmtpReply{code: 250, message: r"2.1.0 OK - smtpd-rs".parse().unwrap()}));
    assert_eq!(dispatch(&"NOOP".to_string()), Some(SmtpReply{code: 250, message: r"2.1.0 OK - smtpd-rs".parse().unwrap()}));
    assert_eq!(dispatch(&"QUIT".to_string()), Some(SmtpReply{code: 221, message: r"2.0.0 closing connection smtpd-rs".to_string()}));
    assert_eq!(dispatch(&"VRFY".to_string()), Some(SmtpReply{code: 250, message: r"2.1.0 OK - smtpd-rs".parse().unwrap()}));

    assert_eq!(dispatch(&"blahblahblah".to_string()), Some(SmtpReply{code: 555, message: r"5.5.2 Syntax error. - smtpd-rs".to_string()}));
}
