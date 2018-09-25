extern crate hostname;
extern crate bufstream;

use bufstream::BufStream;
use std::io;
use std::net::TcpListener;
use std::thread::spawn;
use std::{thread, time};

mod receiver;
mod maildir;

fn main() {
    println!("hello, here is main.");
    smtpd("127.0.0.1:8025").expect("error: ");
}

fn smtpd(addr: &str) -> io::Result<()> {
    receiver::main();
    maildir::main();
    
    spawn(move || {
        loop {
            maildir::scan().err();
            thread::sleep(time::Duration::from_millis(1000));
        }
    });
    
    let listener = TcpListener::bind(addr)?;
    println!("smtpd-rs: listening on {}", addr);
    listener.set_ttl(100).expect("could not set TTL");
    
    loop {
        let (stream, c_addr) = listener.accept()?;
        println!("connection received from {}", c_addr);

        spawn(move || {
            let mut smtp_stream = BufStream::new(stream);
            receiver::handler(&mut smtp_stream).err();
        });
    }
}
