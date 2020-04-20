use std::env;
use syntect::dumps;
use syntect::parsing::SyntaxSetBuilder;

fn help(name: &str) {
    println!("Generate binary dump of SublimeText syntax files. This file can then be included with include_bytes! in binary");
    println!("USAGE: {} path_to_syntax_folder path_to_output", name);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.contains(&"-h".to_string()) || args.contains(&"--help". to_string()) {
        help(&args[0]);
    }

    let mut builder = SyntaxSetBuilder::new();
    if let Err(error) = builder.add_from_folder(&args[1], true) {
        println!("{:?}", error);
        help(&args[0]);
        return
    }

    let syntax_set = builder.build();
    if let Err(error) = dumps::dump_to_file(&syntax_set, &args[2]) {
        println!("{:?}", error);
        help(&args[0]);
    }
}
