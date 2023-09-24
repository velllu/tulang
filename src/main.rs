use parsing::parse_file;
use turing_machine::TuringMachine;

mod opcodes;
mod parsing;
mod turing_machine;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() <= 1 {
        panic!("not enough args");
    }

    let machine = TuringMachine::new(args.get(1).unwrap()).unwrap();
    for state in machine.calculate_states() {
        println!("{}", state);
    }
}
