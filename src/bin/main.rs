extern crate hostname;
extern crate bufstream;

use bufstream::BufStream;
use std::io::{self, Write};
use std::net::TcpListener;
use std::thread::spawn;
use std::{thread, time};
use std::time::{SystemTime, UNIX_EPOCH};
use std::fs::OpenOptions;

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

    spawn(move || {
        thread::sleep(time::Duration::from_millis(1000 * 3));
        loop {
            let start = SystemTime::now();
            let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap();
            let _filename = format!("tests/Maildir/new/mail.{:?}", since_the_epoch);
            println!("filename {}", _filename);

            {
                let mut _file = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create(true)
                    .open(_filename).expect("file create");
                _file.write_all(b"Hello, world!").unwrap();
            }

            thread::sleep(time::Duration::from_millis(1000 * 10));
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
