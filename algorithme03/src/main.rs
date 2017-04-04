fn main() {
    let v = vec![1,2,3];
    my_vec_func(&v);
    my_vec_func(&v);
    
}

fn my_vec_func(v : &Vec<i32>){
    for val in v{
        println!("Vector value is {}", val)
    }
}
