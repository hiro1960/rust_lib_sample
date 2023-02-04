// mod bar;     # こちらはうまくいかない
use bar;

fn main() {
    println!("Hello, world!");

    // 外部のcrateを参照して、使う
    println!("use lib {}", bar::add(1, 2));
}
