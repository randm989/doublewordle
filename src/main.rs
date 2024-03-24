use std::env;

fn main() {
    let _args: Vec<String> = env::args().collect();
    //let program_name = args[0].clone();
    //let json_input = args[1].clone();

    println!("test output of print");
    eprintln!("Test error output of error print");
    let x = 5 + 5;
    eprintln!("Testing x to be equal to {x}");
}
