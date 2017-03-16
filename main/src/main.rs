use std::io;

fn main() {
    println!("数字当てクイズ!");

    println!("番号を入力しよう。");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("読込失敗!!!");

    println!("入力値: {}", guess);
}