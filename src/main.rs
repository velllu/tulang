use parsing::parse_file;

mod opcodes;
mod parsing;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() <= 1 {
        panic!("not enough args");
    }

    let parsed_file = parse_file(args.get(1).unwrap()).unwrap();

    for instruction in parsed_file {
        println!("{}", instruction);
    }
}
