fn main() {
    let v = vec![11,22,33]; 
    take_over(v);

    println!("{}", v);
}

fn take_over(v : Vec<i32>){
    print!("{}", v);
}
