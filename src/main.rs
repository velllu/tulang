use calculation::TuringMachine;

mod calculation;
mod instructions;
mod parsing;

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
