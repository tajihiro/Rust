fn main() {
    let v = vec![1,2,3];
    //let v2 = vec![1,2,3];

    let answer = my_func(&v, &v);

    println!("{:?}", answer);
}

fn my_func(v1: &Vec<i32>, v2: &Vec<i32>) -> i32{
    v1[0] + v2[1]
}

