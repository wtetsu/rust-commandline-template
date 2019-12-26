use futures::executor::block_on;
use std::env;
use tokio::process::Command;

mod lib;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("Usage: {} arg1 [arg2] [arg3] ...", args[0]);
        return;
    }

    lib::plus(1, 2);
    lib::print_arguments(args.clone());

    let task = run(args);
    block_on(task);
}

async fn run(args: Vec<String>) -> std::process::ExitStatus {
    let mut command = Command::new(String::from(args[1].clone()));
    for a in args[2..].iter() {
        command.arg(a);
        println!("{}", a);
    }

    command
        .spawn()
        .expect("error")
        .await
        .expect("ls command failed to run")

    // let status = future.await?;
    // println!("the command exited with: {}", status);
    // println!("{}", std::str::from_utf8(&output.stdout).unwrap());
    // println!("{}", output.status);
}
