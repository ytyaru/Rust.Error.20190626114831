/*
 * Rustで例外処理。エラー委譲。
 * CreatedAt: 2019-06-26
 */
use std::io;
use std::io::Read;
use std::fs::File;
use std::io::{ErrorKind,BufReader,BufWriter,Write};
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
        /*
        Err(ref e) if e.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(mut file) => { file.write("Yamada".as_bytes()).unwrap(); file }, // thread 'main' panicked at 'Os { code: 9, kind: Other, message: "Bad file descriptor" }', main.rs:32:19
                Err(err) => return Err(err),
            }
        },
        Err(e) => return Err(e),
        */
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
fn main() {
    let username = match read_username_from_file() {
        Ok(name) => name,
        Err(e) => panic!("{:?}", e),
    };
    println!("{}", username);
}

