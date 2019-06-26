/*
 * Rustで例外処理。
 * CreatedAt: 2019-06-26
 */
use std::fs::File;

fn main() {
//    let f = File::open("hello.txt");
//    let f:u32 = File::open("hello.txt"); // expected u32, found enum `std::result::Result`
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("ファイルオープンに失敗しました。: {:?}", error),
    };
}

