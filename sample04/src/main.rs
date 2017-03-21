fn main() {
    let my_array = [0,1,2,3,4,5];
    //let all = &my_array[..];
    let middle = &my_array[1..4];

    for val in middle {
        println!("{}", val);
    }    
}
