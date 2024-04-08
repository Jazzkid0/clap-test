// This is a simple example of a CLI application using clap and the derive macro
// It takes a name as an argument and optionally prints a greeting message
// It also optionally prints the first n fibonacci numbers
//
// This should be the general structure of all of my CLI applications

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // The name of the person to greet
    name: String,

    #[arg(short, long)] // This argument is specified with `-v` or `--verbose`
    verbose: bool, // bool is a flag argument type, i.e. it doesn't have a value

    #[arg(short, long)] // This argument is specified with `-f` or `--fibonacci`
    fibonacci: u64, // non bool arguments require a value
}

fn fibonacci(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}

fn main() {
    let args = Args::parse();

    println!("Hello, {}!", args.name);

    if args.verbose {
        println!("I hope you have a great day!");
    }

    if args.fibonacci > 0 {
        for i in 0..args.fibonacci {
            println!("fibonacci({}) = {}", i+1, fibonacci(i));
        }
    }
}
