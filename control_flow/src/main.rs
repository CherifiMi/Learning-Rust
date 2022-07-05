fn main() {



}

fn _stopping_loops(){
    let mut x = 0;

    loop {
        println!("{}", x);
        x +=1;
    }
}

fn _set_val_if(){
    let x = if false {3} else {6};

    println!("{}",x);
}
fn _fizzbuzz(){
    for i in 1..101 {

        if i%3==0 && i%5==0 {println!("FizzBuzz")}
        else if i%3==0 {println!("Fizz")}
        else if i%5==0 {println!("Buzz")}
        else {println!("{i}")}

    }
}

fn ifs(){
    let x = 10;

    if x != 10 {
        println!("Hello");
    }else{
        println!("world!");
    }
}
