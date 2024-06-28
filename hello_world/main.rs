use std::io;
fn main(){
    println!("Hello World");
    let mut str = String::from("");
    io::stdin().read_line(&mut str).
    expect("Unknown error occured!");
    print!("{str}");
}

