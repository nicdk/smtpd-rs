extern crate hostname;
extern crate bufstream;

use std::net::TcpListener;
use std::fs;
use std::path::Path;
use std::io;
use std::thread::spawn;
use bufstream::BufStream;

mod receiver;

fn main() {
    println!("hello, here is main.");
    smtpd("127.0.0.1:8025").expect("error: ");
}

fn smtpd(addr: &str) -> io::Result<()> {
    receiver::main();

    let maildir = fs::read_dir("tests/Maildir")?;
    println!("{:?}", maildir);
    
    for _entry_result in maildir {
        let _entry = _entry_result?;
        if _entry.path().is_dir() && _entry.file_name() == "new" {
            for _entry_new_result in fs::read_dir(_entry.path())? {
                let _entry_new = _entry_new_result?;
                println!("{:?}", _entry_new.path());
                if !_entry_new.path().is_file() {
                    continue;
                }
                let src = _entry_new.path();
                let dst = Path::new("tests/Maildir/tmp/").join(_entry_new.file_name());
                println!("mv: {:?} -> {:?}", src, dst);
                fs::rename(src, dst)?;
            }
        }
    }
    
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
