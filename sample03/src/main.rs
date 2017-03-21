fn main() {
    let answer = plus(2,3);
    println!("{}", answer);
    println!("-----");

    let f0 : fn(i32, i32) -> i32 = plus;
    println!("{}", f0(2,2));
    println!("-----");

    let f1 : fn(i32) -> i32 = plus_one;
    println!("{}", f1(2));
    println!("{}", f1(3));
    println!("{}", f1(4));
    println!("{}", f1(5));
    println!("-----");
}

fn plus(x: i32, y: i32) -> i32{
    x + y
    //return x + y;
}
fn plus_one(x : i32) -> i32{
    x + 1
}

// fn error()-> !{
//     panic!("This Function never Returns!!");
// }
