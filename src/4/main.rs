/*
 * Rustで例外処理。
 * CreatedAt: 2019-06-26
 */
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("ファイルの生成に失敗しました。: {:?}", e),
            }
        },
        Err(error) => panic!("ファイルオープンに失敗しました。: {:?}", error),
    };
}

