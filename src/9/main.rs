/*
 * Rustで例外処理。エラー委譲。`?`演算子はResultを返す関数でしか使えない。
 * CreatedAt: 2019-06-26
 */
use std::fs::File;
fn main() {
    let mut f = File::open("hello.txt")?; // error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `std::ops::Try`)
}

