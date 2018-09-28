extern crate hostname;
extern crate bufstream;

use std::fs;
use std::io;
use std::path::Path;

pub fn main() {
    println!("hello, here is maildir.");
}

pub fn scan() -> io::Result<()> {
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
                } else {
                    let src = _entry_new.path();
                    let dst = Path::new("tests/Maildir/tmp/").join(_entry_new.file_name());
                    println!("mv: {:?} -> {:?}", src, dst);
                    fs::rename(src, dst)?;
                }
            }
        } else {
            println!("{:?}", _entry.file_name());
        }
    }
    Ok(())
}
