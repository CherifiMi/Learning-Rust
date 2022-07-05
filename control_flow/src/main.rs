fn main() {

}

fn _while(){
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

}

fn _labeled_loops(){
    let mut x  = 0;

    'hi: loop {
        //println!("{x}");
        x+=1;
        if x == 10 { break 'hi }

    }

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn _returning_values_from_loops(){
    let mut counter = 0;

    let result = loop {
        println!("{counter}");
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
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
