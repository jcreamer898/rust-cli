use clap::Parser;
use workspaces::get_workspaces;
use subprocess::Exec;
use std::io::{BufRead, BufReader};

mod workspaces;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,

    /// argument to capture the current working directory defaults to current working directory
    /// if not provided
    #[arg(short, long, default_value = ".")]
    cwd: String,
}

fn main() {
    // let args = Args::parse();

    // for _ in 0..args.count {
    //     println!("Hello {}!", args.cwd)
    // }

    // let workspaces = get_workspaces(String::from("./packages/*/package.json"));

    // // iterate through workspaces and print the name
    // for workspace in workspaces {
    //   println!("{}", workspace.packageJson["name"]);
    // }

    let arg = "\"console.log(\'hello\')\"";

    let stream = Exec::cmd("node")
      .arg("-p")
      .arg("-e")
      .arg(arg).stream_stdout().unwrap();

    let br = BufReader::new(stream);
    for (i, line) in br.lines().enumerate() {
        println!("{}: {}", i, line.unwrap());
    }
}
