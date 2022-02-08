use std::io;
pub mod lexer;
pub mod parser;

fn main() {
    loop {
        println!("main>");
        let mut input = String::new(); 
        io::stdin().read_line(&mut input).expect("Invalid Input");
        println!("{}", input);
    }
}
