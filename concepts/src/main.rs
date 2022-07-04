fn main() {
    test2()
}

fn test1() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
}

fn test2(){
    let x: u32 = 1_000;
    let y: u32 = 0xff;

    let tup: (i32, f64, bool) = (500, 6.4, true);
    let (y, x, z) = tup;

    println!(" tup is:{y} and {z} and {x}");
}