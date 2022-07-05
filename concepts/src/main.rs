fn main() {
    functions()
}

fn _var_mut() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
}

fn _data_types(){
    let _x: u32 = 1_000;
    let _y: u32 = 0xff;

    let tup: (i32, f64, bool) = (500, 6.4, true);
    let (y, mut x, z) = tup;

    x = 4.44;

    println!(" tup is:{y} and {z} and {x}");
}

fn _arrays(){
    let a = [0.1, 23.32, 44.3];
    let b = [69; 5];

    for i in b {
        println!("i: {i}")
    }
    println!("{}", a[1])
}

fn functions(){
    let x = {
        let y = 6;
        y*five(3)
    };

    println!("{}", x);
}
/*
will hiiii comments
*/
// * adff
fn five(i: i32) -> i32 {
    5*i
}