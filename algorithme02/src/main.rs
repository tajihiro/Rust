use std::io;

fn main() {
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();
    
    println!("中間値を求めます。");
    println!("a:");
    io::stdin().read_line(&mut a);
    println!("b:");
    io::stdin().read_line(&mut b);
    println!("c:");
    io::stdin().read_line(&mut c);
    let a_num: i32 = a.trim().parse().expect("Need Number");
    let b_num: i32 = b.trim().parse().expect("Need Number");
    let c_num: i32 = c.trim().parse().expect("Need Number");

    let mid:i32 = middle(a_num, b_num, c_num);
    println!("中間値:{}", mid);
}

fn middle(a:i32, b:i32, c:i32)-> i32{
    let mut mid : i32 = 0;
    if a >= b {
        if b >= c {
            mid = b;
        }else if a <= c{
            mid = a;
        }
    }
    else if a > c {
        mid = a;
    }
    else if b > c {
        mid = c;
    }
    else{
        mid = b;
    }
    mid 
}
