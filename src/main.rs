use clap::{CommandFactory, Parser};
use std::{
  fs::File,
  io::{stdin, BufRead, BufReader, IsTerminal},
  path::PathBuf,
};

/// A simple grep-like tool
#[derive(Parser)]
#[command(arg_required_else_help = true)]
struct Cli {
  /// The path to the file to read, use - to read from stdin (must not be a tty)
  file: PathBuf,
  /// The pattern to search for in the file
  pattern: String,
}

fn main() {
  let args = Cli::parse();

  let file = args.file;
  let pattern = args.pattern;

  if file == PathBuf::from("-") {
    if stdin().is_terminal() {
      Cli::command().print_help().unwrap();
      ::std::process::exit(2);
    }

    file_grepper(&pattern, BufReader::new(stdin())).unwrap();
  } else {
    let file = File::open(&file).unwrap();
    let buf_reader = BufReader::new(file);
    file_grepper(&pattern, buf_reader).unwrap();
  }
}

fn file_grepper<R: BufRead>(pattern: &str, buf_reader: R) -> std::io::Result<()> {
  for (line, i) in buf_reader.lines().zip(1..) {
    let line = line?;
    if line.contains(pattern) {
      let line = line.replace(pattern, &format!("\x1b[31m{}\x1b[0m", pattern));
      let line_no = format!("\x1b[32m{}\x1b[0m", i);
      println!("{}: {}", line_no, line);
    }
  }
  Ok(())
}
