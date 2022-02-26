use std::{
    env,
    fs::{self, File},
    io::prelude::*,
    str::FromStr,
};

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

impl FromStr for Cmd {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
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
            _ => match s.parse::<i32>() {
                Ok(num) => Ok(Self::IntLit(num)),
                Err(_) => Err("invalid postfix command"),
            },
        }
    }
}

impl Cmd {
    fn to_asm(&self) -> String {
        match self {
            Self::IntLit(num) => format!("push {}\n", num),
            Self::Swap => {
                "mov rax, qword[rsp]\nxchg rax, qword[rsp + 8]\nmov qword[rsp], rax\n".to_string()
            }
            Self::Sel => "nop\n".to_string(),
            Self::Cmp(rel) => [
                "xor rdx, rdx\nmov rdi, 1\npop rax\npop rcx\ncmp rcx, rax\n",
                match rel {
                    Cmp::Gt => "cmovg rdx, rdi\n",
                    Cmp::Lt => "cmovl rdx, rdi\n",
                    Cmp::Eq => "cmove rdx, rdi\n",
                },
                "push rdx\n",
            ]
            .concat(),
            Self::Math(arith) => [
                "pop rax\npop rcx\n",
                match arith {
                    Math::Add => "add rax, rcx\n",
                    Math::Sub => "sub rax, rcx\n",
                    Math::Mul => "mul rcx\n",
                    Math::Div => "xor rdx, rdx\ndiv rcx\n",
                },
                "push rax\n",
            ]
            .concat(),
            Self::Exec => "pop rax\ncall rax\n".to_string(),
            Self::Nget => "nop\n".to_string(),
            Self::Seq(_) => "nop\n".to_string(),
        }
    }
}

fn parse_seq(tokens: &mut dyn Iterator<Item = &String>) -> Result<Vec<Cmd>, &'static str> {
    let mut cmd_stack = Vec::new();

    while let Some(cmd) = tokens.next() {
        if cmd == ")" {
            break;
        } else if cmd == "(" {
            match parse_seq(tokens) {
                Ok(seq) => cmd_stack.push(Cmd::Seq(seq)),
                Err(err) => return Err(err),
            }
        } else {
            match Cmd::from_str(cmd) {
                Ok(cmd) => cmd_stack.push(cmd),
                Err(err) => return Err(err),
            }
        }
    }
    Ok(cmd_stack)
}

#[derive(Debug)]
#[allow(dead_code)]
enum Arg {
    IntLit(i32),
    Seq(Vec<Cmd>),
}

#[derive(Debug)]
struct Program {
    stack: Vec<Arg>,
    cmds: Vec<Cmd>,
}

impl Program {
    fn new(cmds: String) -> Result<Self, &'static str> {
        let mut tokens_vec: Vec<&str> = Vec::new();
        let mut offset = 0;
        loop {
            if let Some((idx, paren)) = match (cmds[offset..].find("("), cmds[offset..].find(")")) {
                (Some(idx1), Some(idx2)) => {
                    if idx1 < idx2 {
                        Some((idx1, "("))
                    } else {
                        Some((idx2, ")"))
                    }
                }
                (Some(idx), None) => Some((idx, "(")),
                (None, Some(idx)) => Some((idx, ")")),
                _ => None,
            } {
                tokens_vec.push(&cmds[offset..(offset + idx)]);
                tokens_vec.push(paren);
                offset += idx + 1;
            } else {
                tokens_vec.push(&cmds[offset..]);
                break;
            }
        }

        let tokens_vec: Vec<String> = tokens_vec
            .iter()
            .map(|s| s.split_whitespace())
            .flatten()
            .map(|s| s.to_string())
            .collect();
        let mut tokens = tokens_vec.iter();
        if tokens_vec.len() >= 3
            && (tokens.next().unwrap().clone() + tokens.next().unwrap()).as_str() == "(postfix"
        {
            if let Ok(n_args) = tokens.next().unwrap().parse::<usize>() {
                match parse_seq(&mut tokens) {
                    Ok(cmds) => {
                        let mut stack = Vec::new();
                        for _ in 0..n_args {
                            match tokens.next().map(|s| s.parse::<i32>()) {
                                Some(Ok(n)) => stack.push(Arg::IntLit(n)),
                                _ => return Err("invalid stack arguments"),
                            }
                        }
                        if stack.len() != n_args {
                            return Err("missing program arguments");
                        }

                        Ok(Self { stack, cmds })
                    }
                    Err(err) => Err(err),
                }
            } else {
                Err("invalid postfix program")
            }
        } else {
            Err("invalid postfix program")
        }
    }

    fn compile(&self) -> String {
        let mut result = "global _start\nsection .text\n_start:\n".to_string();

        for cmd in &self.cmds {
            result += &cmd.to_asm();
        }

        result + "mov rax, 60\nxor rdi, rdi\nsyscall\n"
    }
/*
    fn simulate(&self) -> Result<(), &'static str> {
        Ok(())
    }
*/
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("no file provided");
    }

    let contents = fs::read_to_string(&args[1]).expect("problem reading from file");

    match Program::new(contents) {
        Ok(program) => {
            let mut file = File::create("main.asm")?;
            write!(file, "{}", program.compile())?;
            println!("{:?}\n{:?}", program.stack, program.cmds);
        }
        Err(msg) => println!("{}", msg),
    }

    Ok(())
}
