fn main() {

    let s = String::from("sdfsdfw dfw eefs fds ewerr");

    println!("{}", first_word(&s))

}

fn first_word(s: &String)->usize{

    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return i
        }
    }

    s.len()

}

//_____________________________________________________________________
fn _ref_scope(){
    let mut s = String::from("hello");

    let r3 = &mut s;
    println!("{} and {}", r3, "r2");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
}
//____________________________________________________________________
fn _ref(){
    let mut s = String::from("hillo");
    let len = _calculate_length(&mut s);

    println!("The length of '{}' is {}.", s, len);
}
fn _calculate_length(s: &mut String) -> usize{
    s.push_str("there");
    s.len()
}
//____________________________________________________________________
fn _return_scope(){
    let s1 = _gives_ownership();

    let s2 = String::from("hello");

    let s3 = _takes_and_gives_back(s2);

    println!("{}", s3)
}
fn _takes_and_gives_back(a_string: String) -> String {
    a_string
}
fn _gives_ownership() -> String {


    let some_string = String::from("yours");

    some_string
}
//_______________________________________________________________________
fn _copy_clone_move() {
    let s1 = String::from("well hello there!!");
    let s2 = s1;

    let z1 = String::from("well hello there!!");
    let z2 = z1.clone();

    let x = 1;
    let y = x;

    let d = 4;
    let b = d;

    println!("  z:{}  s:{} d:{}", z1 == z2, x == y, d == b);
}
//_______________________________________________________________________
fn _functions() {
    let s = String::from("mito");

    _takes_ownership(s);

    let x = 5;
    _make_copy(x);

    println!("{}", x);
}
fn _takes_ownership(s: String) {
    println!("{}", s);
}
fn _make_copy(x: i32) {
    println!("{}", x);
}
