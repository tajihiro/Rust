fn main() {

    let s1 = "Hello";
    let s2;
    s2 = "たぢさん!";

    println!("{}, {}", s1, s2);


    let mut num = 3;
    println!("A : {}", num);
    num = 2;
    println!("B : {}", num);

    let (num1, num2) = (4,5);

    println!("変数1 : {}", myfunc(num,2));
    println!("変数2 : {}", myfunc(num1,num2));

    let a = vec![4,9];

    println!("{:?}", a[0]);

    println!("{:?}", my_func_with_ref(&a));
    println!("{:?}", my_func_with_ref(&a));

    let mut v = vec![101,102,103];
    v.push(111);
    for i in v{
        println!("{}", i);
        //v.push(111);
    } 
}

fn myfunc(x:i32, y:i32) -> i32{
    x + y
}

fn my_func_with_ref(x: &[i32]) -> i32{
    x[0] + x[1]
}
