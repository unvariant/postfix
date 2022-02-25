use std::env;
use std::fs;

enum Arg {
    Literal(i32),
    Instr(Instr),
}

impl TryFrom<&str> for Arg {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match Instr::try_from(s) {
            Ok(instr) => Ok(Arg::Instr(instr)),
            Err(_) => {
                match s.parse() {
                    Ok(num) => Ok(Arg::Literal(num)),
                    Err(_) => Err("(postfix): invalid argument"),
                }
            },
        }
    }
}

enum Instr {
    Add,
    Sub,
    Mul,
    Output,
}

impl TryFrom<&str> for Instr {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "add" => Ok(Instr::Add),
            "sub" => Ok(Instr::Sub),
            "mul" => Ok(Instr::Mul),
            "output" => Ok(Instr::Output),
            _ => Err("(postfix): invalid instruction"),
        }
    }
}

impl Instr {
    fn add(stack: &mut Vec<i32>) -> Result<(), &'static str>{
        if stack.len() >= 2 {
            let arg1 = stack.pop().unwrap();
            let arg2 = stack.pop().unwrap();
            stack.push(arg2 + arg1);
            Ok(())
        } else {
            Err("(postfix): missing arguments on stack")
        }
    }

    fn sub(stack: &mut Vec<i32>) -> Result<(), &'static str>{
        if stack.len() >= 2 {
            let arg1 = stack.pop().unwrap();
            let arg2 = stack.pop().unwrap();
            stack.push(arg2 - arg1);
            Ok(())
        } else {
            Err("(postfix): missing arguments on stack")
        }
    }

    fn mul(stack: &mut Vec<i32>) -> Result<(), &'static str>{
        if stack.len() >= 2 {
            let arg1 = stack.pop().unwrap();
            let arg2 = stack.pop().unwrap();
            stack.push(arg2 * arg1);
            Ok(())
        } else {
            Err("(postfix): missing arguments on stack")
        }
    }

    fn output(stack: &mut Vec<i32>) -> Result<(), &'static str> {
        if stack.len() >= 1 {
            print!("{}", *stack.last().unwrap() as u8 as char);
            Ok(())
        } else {
            Err("(postfix): missing arguments on stack")
        }
    }
}

fn run(cmd: String, mut stack: Vec<i32>) -> Vec<i32> {
    let mut cmd = cmd + " ";

    while let Some(space) = cmd.find(" ") {
        match Arg::try_from(&cmd[..space]) {
            Ok(arg) => {
                match arg {
                    Arg::Literal(num) => stack.push(num),
                    Arg::Instr(instr) => match match instr {
                        Instr::Add => Instr::add(&mut stack),
                        Instr::Sub => Instr::sub(&mut stack),
                        Instr::Mul => Instr::mul(&mut stack),
                        Instr::Output => Instr::output(&mut stack),
                    } {
                        Ok(_) => {},
                        Err(err) => panic!("{}", err),
                    }
                };
                cmd = cmd[space..].trim_start().to_string();
            },
            Err(err) => panic!("{}", err),
        };
    }

    stack
}

fn main() {
    let mut stack: Vec<i32> = Vec::new();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("(postfix): no file provided");
    }

    let contents = fs::read_to_string(&args[1])
        .expect("(postfix): problem reading from file");
    
    for line in contents.split("\n") {
        if line.starts_with("(postfix") && line.ends_with(")") {
            stack = run(line[9..line.len() - 1].trim().to_string(), stack);
        } else {
            panic!("(postfix): invalid postfix command");
        }
    }
}