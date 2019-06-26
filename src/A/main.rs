/*
 * Rustで例外処理。エラー委譲。`?`演算子はResultを返す関数でしか使えない。
 * CreatedAt: 2019-06-26
 */
pub struct Guess {
    value: u32,
}
impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 { panic!("入力値は1〜100の整数にしてください。入力値: {}", value); }
        Guess { value }
    }
    pub fn value(&self) -> u32 { self.value }
}
fn main() {
    Guess::new(1);
    Guess::new(101);
}

