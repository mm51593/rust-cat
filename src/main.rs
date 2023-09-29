use std::{fs::File, env::args, io::{BufReader, BufRead}};

#[derive(Default)]
struct GlobalState {
    number: u64,
}

enum Flag {
    Default,
    Numbers
}

impl From<&String> for Flag {
    fn from(value: &String) -> Self {
        match value.as_str() {
            "-n" => Self::Numbers,
            _ => Self::Default
        }
    }
}

struct Arguments {
    files: Vec<String>,
    numbers: bool,
}

#[derive(Default)]
struct ArgumentsBuilder {
    files: Vec<String>,
    numbers: bool,
}

impl ArgumentsBuilder {
    pub fn file(&mut self, file: String) {
        self.files.push(file);
    }

    pub fn number(&mut self) {
        self.numbers = true;
    }

    pub fn build(self) -> Arguments {
        Arguments { files: self.files, numbers: self.numbers }
    }
}

fn main() {
    let mut global_state = GlobalState::default();
    let arguments = get_args();

    for file in arguments.files.iter() {
        let file = File::open(file);

        match file {
            Ok(f) => process_file(f, &arguments, &mut global_state),
            Err(e) => eprintln!("{e}"),
        }        
    }
}

fn get_args() -> Arguments {
    let mut builder = ArgumentsBuilder::default();
    let state = Flag::Default;

    for arg in args().skip(1) {
        match state {
            Flag::Default => {
                let flag = Flag::from(&arg);
                match flag {
                    Flag::Default => builder.file(arg),
                    Flag::Numbers => builder.number(),
                }
            },
            _ => unreachable!()
        }
    }

    builder.build()
}

fn process_file(file: File, arguments: &Arguments, global_state: &mut GlobalState) {
    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(l) => print_line(&l, arguments, global_state),
            Err(e) => eprintln!("{e}"),
        }
    }
}

fn print_line(line: &str, arguments: &Arguments, global_state: &mut GlobalState) {
    global_state.number += 1;
    if arguments.numbers {
        print!("{}\t", global_state.number);
    }
    println!("{line}");
}