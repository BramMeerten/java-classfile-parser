mod parser;

use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    let mut file = "./java/Main.class";
    if args.len() == 2 {
        file = &args[1];
    }

    let class = parser::parse_file(file);
    println!("{:#?}", class);
}
