use std::{env, fs, str::FromStr};

#[derive(Debug)]
enum Math {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
enum Cmp {
    Gt,
    Lt,
    Eq,
}

#[derive(Debug)]
enum Cmd {
    IntLit(i32),
    Swap,
    Sel,
    Cmp(Cmp),
    Math(Math),
    Exec,
    Nget,
    Seq(Vec<Cmd>),
}

impl Cmd {
    fn from_str(s: &str) -> Result<Self, &'static str> {
        println!("{}", s);
        match s {
            "swap" => Ok(Self::Swap),
            "sel" => Ok(Self::Sel),
            "gt" | ">" => Ok(Self::Cmp(Cmp::Gt)),
            "lt" | "<" => Ok(Self::Cmp(Cmp::Lt)),
            "eq" | "==" => Ok(Self::Cmp(Cmp::Eq)),
            "add" | "+" => Ok(Self::Math(Math::Add)),
            "sub" | "-" => Ok(Self::Math(Math::Sub)),
            "mul" | "*" => Ok(Self::Math(Math::Mul)),
            "div" | "/" => Ok(Self::Math(Math::Div)),
            "exec" => Ok(Self::Exec),
            "nget" => Ok(Self::Nget),
            _ => {
                match s.parse::<i32>() {
                    Ok(num) => Ok(Self::IntLit(num)),
                    Err(_) => Err("invalid postfix command"),
                }
            },
        }
    }
}

fn parse_seq(s: &str, mut offset: usize) -> Result<Vec<Cmd>, &'static str> {
    let mut cmd_stack = Vec::new();

    while &s[offset..(offset + 1)] != ")" {
        if &s[offset..(offset + 1)] == "(" {
            match parse_seq(&s, offset) {
                Ok(seq) => cmd_stack.push(Cmd::Seq(seq)),
                Err(err) => return Err(err),
            }
        } else if let Some(idx) = s[offset..].find(" ") {
            match Cmd::from_str(&s[offset..(offset + idx)]) {
                Ok(cmd) => {
                    cmd_stack.push(cmd);
                    offset += idx + 1;
                },
                Err(err) => return Err(err),
            }
        }
        offset += 1;
        if offset == s.len() - 1 {
            return Err("invalid command sequence");
        }
    }
    Ok(cmd_stack)
}

#[derive(Debug)]
enum Arg {
    IntLit(i32),
    Seq(Vec<Cmd>),
}

fn parse(s: &str) -> Option<(i32, usize)> {
    let mut num = 0;
    let mut offset = 0;

    for ch in s.chars() {
        if ch > '0' && ch < '9' {
            num = num * 10 + ch as i32 - 48;
            offset += 1;
        } else {
            break;
        }
    };

    if offset == 0 {
        None
    } else {
        Some((num, offset))
    }
}

#[derive(Debug)]
struct Program {
    stack: Vec<Arg>,
    cmds: Vec<Cmd>,
}

impl Program {
    fn new(cmds: String) -> Result<Self, &'static str> {
        if cmds.starts_with("(") && cmds.ends_with(")") && cmds[1..].trim().starts_with("postfix")
        {
            let mut cmds = cmds[1..(cmds.len() - 1)].trim_start()[7..].trim_start().to_string();
            if let Some((num, offset)) = parse(&cmds) {     
                println!("<{}>", &cmds[(offset + 1)..]);           
                match parse_seq(&cmds, offset + 1) {
                    Ok(seq) => Ok(Self {
                        stack: Vec::new(),
                        cmds: seq,
                    }),
                    Err(err) => Err(err),
                }
            } else {
                Err("number of program arguments not specified")
            }
        } else {
            Err("invalid postfix program")
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("no file provided");
    }

    let contents = fs::read_to_string(&args[1]).expect("problem reading from file");

    match Program::new(contents) {
        Ok(program) => println!("{:?}", program),
        Err(msg) => println!("{}", msg),
    }
}
