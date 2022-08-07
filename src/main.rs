use std::io;

fn main() {

    let mut new = String::new();
    let mut old = String::new();
    println!("Input New Number:");
    io::stdin()
        .read_line(&mut new)
        .expect("Fail");
    
    println!("Original Number");
    io::stdin()
        .read_line(&mut old)
        .expect("Fail");
    
    let old_int = old.parse::<i32>();
    let new_int = new.parse::<i32>();

    let inc = new_int - old_int;

    print!("{}", inc);
}
