fn main() {

    let s = "Hello, たぢさん!";
    println!("{}",s);

    let mut num = 3;
    println!("A:{}", num);
    num = 2;
    println!("B:{}", num);

    println!("{}", myfunc(3,2));
}

fn myfunc(x:i32, y:i32) -> i32{
    x + y
}
