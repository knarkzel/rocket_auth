








fn func(x: i32) -> i32 {


    println!("x: {:p}", &x);

    let out = 2 * x;

    println!("out: {:p}", &out);

    out

}




fn main() {





    let i: i32 = 3;
    for i in 0..100 {
        func(i);
    }
    

    let j: i32 = func(i);
    
    
    println!("
    i: {:#p}
    j: {:#p}
    ", &i, &j);
}

