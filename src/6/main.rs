/*
 * Rustで例外処理。
 * CreatedAt: 2019-06-26
 */
use std::fs::File;
fn main() {
    let f = File::open("hello.txt").expect("ファイルオープンに失敗しました。");
}

