use std::io;

fn main() {
    println!("1-1:");
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();
    println!("最大値を求めます。");
    println!("a:");
    io::stdin().read_line(&mut a);
    println!("b:");
    io::stdin().read_line(&mut b);
    println!("c:");
    io::stdin().read_line(&mut c);
    let a_num: i32 = a.trim().parse().expect("Need Number");
    let b_num: i32 = b.trim().parse().expect("Need Number");
    let c_num: i32 = c.trim().parse().expect("Need Number");
    
    let mut max: i32 = a_num;
    if max < b_num {
        max = b_num;
    }
    if max < c_num {
        max = c_num;
    }
    
    println!("最大値:{}", max);
}
