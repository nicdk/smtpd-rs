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
struct ReceiveMail {
    pub mail_from: String,
    pub rcpt_to: String,
    pub data: String
}

impl ReceiveMail {
    pub fn init() -> ReceiveMail {
        ReceiveMail {
            mail_from: String::new(),
            rcpt_to: String::new(),
            data: String::new()
        }
    }
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
    let mut _mail = ReceiveMail::init();

    loop {
        state = match &state {
            SmtpState::INIT => {
                let mut line = String::new();
                if let Err(err) = stream.read_line(&mut line) {
                    print_error(&err);
                    panic!("error: {}", err);
                }
                println!("line {:?}", line.trim());

                let _cmd;
                match line.trim().split_whitespace().nth(0) {
                    Some(s) => _cmd = s,
                    None => {
                        stream.write(format!("555 5.5.2 Syntax error. - smtpd-rs\n").as_bytes())?;
                        stream.flush()?;
                        continue;
                    }
                };
                println!("command {}", _cmd);

                match dispatch(&line.trim().to_string()) {
                    Some(_reply) => {
                        let _code = &_reply.code;
                        let _message: &str = &_reply.message;
                        stream.write(&_message.as_bytes())?;
                        stream.flush()?;

                    }
                    None => {
                        /* abort. */
                        stream.write(b"disconnect.")?;
                        stream.flush()?;
                        panic!();
                    }
                }

                match &_cmd {
                    &"HELO" => SmtpState::READY,
                    &"EHLO" => SmtpState::READY,
                    &"RSET" => SmtpState::READY,
                    &"QUIT" => {
                        /* abort. */
                        stream.write(b"disconnect.\n")?;
                        stream.flush()?;
                        panic!();
                    },
                    _ => SmtpState::INIT,
                }
            }
            SmtpState::READY => {
                let mut line = String::new();
                if let Err(err) = stream.read_line(&mut line) {
                    print_error(&err);
                    panic!("error: {}", err);
                }
                println!("line {:?}", line.trim());

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

                match dispatch(&line.trim().to_string()) {
                    Some(_reply) => {
                        let _code = &_reply.code;
                        let _message: &str = &_reply.message;
                        stream.write(&_message.as_bytes())?;
                        stream.flush()?;

                        _mail.mail_from = match line.trim().split_whitespace().nth(1) {
                            Some(s) => String::from(s),
                            None => String::new(),
                        };
                    }
                    None => {
                        /* abort. */
                        stream.write(b"disconnect.")?;
                        stream.flush()?;
                        panic!();
                    }
                }

                match &_cmd {
                    &"MAIL" => SmtpState::MAIL,
                    &"RSET" => SmtpState::READY,
                    &"QUIT" => {
                        /* abort. */
                        stream.write(b"disconnect.\n")?;
                        stream.flush()?;
                        panic!();
                    },
                    _ => SmtpState::READY,
                }
            }
            SmtpState::MAIL => {
                let mut line = String::new();
                if let Err(err) = stream.read_line(&mut line) {
                    print_error(&err);
                    panic!("error: {}", err);
                }
                println!("line {:?}", line.trim());

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

                match dispatch(&line.trim().to_string()) {
                    Some(_reply) => {
                        let _code = &_reply.code;
                        let _message: &str = &_reply.message;
                        stream.write(&_message.as_bytes())?;
                        stream.flush()?;

                        _mail.rcpt_to = match line.trim().split_whitespace().nth(1) {
                            Some(s) => String::from(s),
                            None => String::new(),
                        };
                    }
                    None => {
                        /* abort. */
                        stream.write(b"disconnect.")?;
                        stream.flush()?;
                        panic!();
                    }
                }

                match &_cmd {
                    &"RCPT" => SmtpState::RCPT,
                    &"RSET" => SmtpState::READY,
                    &"QUIT" => {
                        /* abort. */
                        stream.write(b"disconnect.\n")?;
                        stream.flush()?;
                        panic!();
                    },
                    _ => SmtpState::MAIL,
                }
            }
            SmtpState::RCPT => {
                let mut line = String::new();
                if let Err(err) = stream.read_line(&mut line) {
                    print_error(&err);
                    panic!("error: {}", err);
                }
                println!("line {:?}", line.trim());

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

                match dispatch(&line.trim().to_string()) {
                    Some(_reply) => {
                        let _code = &_reply.code;
                        let _message: &str = &_reply.message;
                        stream.write(&_message.as_bytes())?;
                        stream.flush()?;

                    }
                    None => {
                        /* abort. */
                        stream.write(b"disconnect.\n")?;
                        stream.flush()?;
                        panic!();
                    }
                }

                match &_cmd {
                    &"DATA" => SmtpState::DATA,
                    &"RSET" => SmtpState::READY,
                    &"QUIT" => {
                        /* abort. */
                        stream.write(b"disconnect.\n")?;
                        stream.flush()?;
                        panic!();
                    },
                    _ => SmtpState::RCPT,
                }
            }
            SmtpState::DATA => {
                let mut _data = Vec::<String>::new();
                loop {
                    let mut _l = String::new();
                    if let Err(e) = stream.read_line(&mut _l) {
                        print_error(&e);
                        continue;
                    }
                    println!("line {:?}", _l.trim());

                    if _l.trim().to_string() == "." { break; }
                    if _l.trim().to_string() == ".."  { _data.push(".".to_string()); continue; }

                    _data.push(_l);
                }
                _mail.data = _data.concat();

                if _mail.mail_from.is_empty() {
                    println!("mail from is empty.");
                    stream.write(format!("451 Reequested action aborted. FROM address is empty.\n").as_bytes())?;
                    stream.flush()?;
                } else if _mail.rcpt_to.is_empty() {
                    println!("rcpt to is empty.");
                    stream.write(format!("451 Reequested action aborted.RCPT address is empty.\n").as_bytes())?;
                    stream.flush()?;
                } else if _mail.data.is_empty() {
                    println!("data is empty.");
                    stream.write(format!("451 Reequested action aborted. DATA context is empty.\n").as_bytes())?;
                    stream.flush()?;
                } else {
                    println!("write mail. {:?}", _mail);
                    stream.write(format!("250 OK\n").as_bytes())?;
                    stream.flush()?;
                }

                SmtpState::READY
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
                        message: format!("5.5.2 Syntax error. - smtpd-rs\n")
                    }
                }
                Some(param1) => {
                    if command_split.next() != None {
                        reply = SmtpReply{
                            code: 555,
                            message: format!("5.5.2 Syntax error. - smtpd-rs\n")
                        };
                    } else {
                        reply = SmtpReply{
                            code: 250,
                            message: format!("ok at your service {:?}\n", param1)
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
                        message: format!("5.5.2 Syntax error. - smtpd-rs\n")
                    }
                }
                Some(param1) => {
                    if command_split.next() != None {
                        reply = SmtpReply{
                            code: 555,
                            message: format!("5.5.2 Syntax error. - smtpd-rs\n")
                        };
                    } else {
                        reply = SmtpReply{
                            code: 250,
                            message: format!("ok at your service {:?}\n", param1)
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
                message: format!("2.1.0 OK - smtpd-rs\n")
            };
            Some(reply)
        }
        Some("RCPT") => {
            let rcpt_address = command_split.next();
            println!("{:?}", rcpt_address);
            let reply = SmtpReply{
                code: 250,
                message: format!("2.1.0 OK - smtpd-rs\n")
            };
            Some(reply)
        }
        Some("DATA") => {
            let reply = SmtpReply{
                code: 354,
                message: format!("Start mail input; end with <CRLF>.<CRLF>\n")
            };
            Some(reply)
        }
        Some("RSET") => {
            let reply = SmtpReply{
                code: 250,
                message: format!("2.1.0 OK - smtpd-rs\n")
            };
            Some(reply)
        }
        Some("NOOP") => {
            let reply = SmtpReply{
                code: 250,
                message: format!("2.1.0 OK - smtpd-rs\n")
            };
            Some(reply)
        }
        Some("QUIT") => {
            let reply = SmtpReply{
                code: 221,
                message: format!("Closing connection. Good bye.\n")
            };
            Some(reply)
        }
        Some("VRFY") => {
            let reply = SmtpReply{
                code: 250,
                message: format!("2.1.0 OK - smtpd-rs\n")
            };
            Some(reply)
        }
        _ => {
            let reply = SmtpReply{
                code: 555,
                message: format!(r"5.5.2 Syntax error. - smtpd-rs\n")
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
    assert_eq!(dispatch(&"EHLO localhost".to_string()), Some(SmtpReply{ code: 250, message: "ok at your service \"localhost\"\n".parse().unwrap() }));
    assert_eq!(dispatch(&"HELO localhost".to_string()), Some(SmtpReply{ code: 250, message: "ok at your service \"localhost\"\n".parse().unwrap() }));

    assert_eq!(dispatch(&"MAIL FROM:postmaster@example.com".to_string()), Some(SmtpReply{ code: 250, message: "2.1.0 OK - smtpd-rs\n".parse().unwrap() }));
    assert_eq!(dispatch(&"RCPT TO:postmaster@example.com".to_string()),   Some(SmtpReply{ code: 250, message: "2.1.0 OK - smtpd-rs\n".parse().unwrap() }));
    assert_eq!(dispatch(&"DATA".to_string()), Some(SmtpReply { code: 354, message: "Start mail input; end with <CRLF>.<CRLF>\n".parse().unwrap() }));

    assert_eq!(dispatch(&"RSET".to_string()), Some(SmtpReply{code: 250, message: "2.1.0 OK - smtpd-rs\n".parse().unwrap() }));
    assert_eq!(dispatch(&"NOOP".to_string()), Some(SmtpReply{code: 250, message: "2.1.0 OK - smtpd-rs\n".parse().unwrap() }));
    assert_eq!(dispatch(&"QUIT".to_string()), Some(SmtpReply{code: 221, message: "Closing connection. Good bye.\n".to_string() }));
    assert_eq!(dispatch(&"VRFY".to_string()), Some(SmtpReply{code: 250, message: "2.1.0 OK - smtpd-rs\n".parse().unwrap() }));

    assert_eq!(dispatch(&"blahblahblah".to_string()), Some(SmtpReply{code: 555, message: r"5.5.2 Syntax error. - smtpd-rs\n".to_string() }));
}
