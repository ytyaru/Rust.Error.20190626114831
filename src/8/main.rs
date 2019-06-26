/*
 * Rustで例外処理。エラー委譲。`?`演算子でエラーを呼出元へ返す。
 * CreatedAt: 2019-06-26
 */
use std::io;
use std::io::{Read,ErrorKind};
use std::fs::File;

fn read_username_from_file() -> Result<String, std::io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s);
    Ok(s)
}
/*
use std::io;
use std::io::Read;
use std::fs::File;
use std::io::{ErrorKind,BufReader,BufWriter,Write};
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
*/
fn main() {
    let username = match read_username_from_file() {
        Ok(name) => name,
        Err(e) => panic!("{:?}", e),
    };
    println!("{}", username);
}

