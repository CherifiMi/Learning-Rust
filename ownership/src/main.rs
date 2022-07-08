fn main() {


}






//_______________________________________________________________________-
fn _functions(){
    let s = String::from("mito");

    takes_ownership(s);

    let x = 5;
    make_copy(x);

    println!("{}", x);
}
fn takes_ownership(s: String){
    println!("{}", s);
}
fn make_copy(x: i32){
    println!("{}", x);
}


fn _copy_clone_move(){
    let s1 = String::from("well hello there!!");
    let s2 = s1;

    let z1 = String::from("well hello there!!");
    let z2 = z1.clone();

    let x = 1;
    let y = x;

    let d = 4;
    let b = d;

    println!("  z:{}  s:{} d:{}", z1==z2, x==y, d==b);
}