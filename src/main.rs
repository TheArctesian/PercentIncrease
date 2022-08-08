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
    
    let old_int = old.trim_end().parse::<f32>().expect("wrong fmt");
    let new_int = new.trim_end().parse::<f32>().expect("wrong fmt");

    let inc = new_int - old_int;

    println!("Increase: {}", inc);

    //let per_inc = inc / old_int;
    println!("Percent Increase {:.3}", inc/old_int*100.0);
    // Norm 
    // 19954 11766
    // 
    // Random
    // 32308
    // 
    // DaddyBayes
    // 132525
    //
    // Tec
    // 24260
}
