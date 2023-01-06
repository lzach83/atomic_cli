use atomic_cli::Selections;
use std::error::Error;
use std::fs;
mod logo;
fn main() {
    let print_logo = logo::Logo::show();
    println!("{}", print_logo);
    Selections::new()
}
