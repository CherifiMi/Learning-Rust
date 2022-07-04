fn main() {
    test2()
}

fn _test1() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
}

fn test2(){
    let _x: u32 = 1_000;
    let _y: u32 = 0xff;

    let tup: (i32, f64, bool) = (500, 6.4, true);
    let (y, mut x, z) = tup;

    x = 4.44;

    println!(" tup is:{y} and {z} and {x}");
}