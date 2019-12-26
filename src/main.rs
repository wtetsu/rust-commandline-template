use std::env;
mod lib;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("Usage: {} arg1 [arg2] [arg3] ...", args[0]);
        return;
    }

    lib::plus(1, 2);

    lib::print_arguments(args);
}
