use std::{env, fs};

fn arithmetic(cmd: &str, stack: &mut Vec<Arg>) {
    if stack.len() >= 2 {
        if let (Some(Arg::Lit(a1)), Some(Arg::Lit(a2))) = (stack.pop(), stack.pop()) {
            stack.push(Arg::Lit(match cmd {
                "add" | "+" => a2 + a1,
                "sub" | "-" => a2 - a1,
                "mul" | "*" => a2 * a1,
                "div" | "/" => a2 / a1,
                _ => {
                    println!("invalid arithmetic command");
                    0
                }
            }));
        } else {
            println!("mismatched types on stack");
        }
    } else {
        println!("missing arguments on stack");
    }
}

#[derive(Debug)]
enum Arg {
    Lit(i32),
    Seq(String),
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("no file provided");
    }

    let contents = fs::read_to_string(&args[1]).expect("problem reading from file");

    if contents.starts_with("(") && contents.ends_with(")") {
        if contents[1..].trim_start().starts_with("postfix") {
            let cmds = contents[(contents.find("postfix").unwrap() + 7)..(contents.len() - 1)]
                .trim()
                .to_string()
                + " ";
            let mut stack: Vec<Arg> = vec![Arg::Seq(cmds)];
            exec(&mut stack);
            println!("{:?}", stack.last());
        } else {
            panic!("invalid postfix program");
        }
    }
}

fn try_run(cmds: String, stack: &mut Vec<Arg>) -> Result<String, &'static str> {
    let mut cmds = cmds + " ";
    while cmds.len() > 0 {
        if cmds.starts_with("(") {
            let mut parens: Vec<char> = Vec::new();
            if let Some(idx) = cmds.chars().enumerate().find_map(|(idx, ch)| {
                if ch == '(' {
                    parens.push('(');
                    None
                } else if ch == ')' {
                    parens
                        .pop()
                        .and_then(|_| if parens.is_empty() { Some(idx) } else { None })
                } else {
                    None
                }
            }) {
                stack.push(Arg::Seq(cmds[1..idx].to_string()));
                cmds = cmds[(idx + 1)..].trim_start().to_string();
            } else {
                return Err("invalid command sequence");
            }
        } else if let Some(idx) = cmds.find(" ") {
            let idx = std::cmp::min(idx, cmds.find("\n").unwrap_or(usize::MAX));
            let cmd = &cmds[..idx];
            if cmds.trim() == "" {
                return Ok("".to_string());
            }
            match cmd {
                "add" | "+" | "sub" | "-" | "mul" | "*" | "div" | "/" => arithmetic(cmd, stack),
                "swap" => swap(stack),
                "exec" => exec(stack),
                "skip" => {}
                _ => match cmd.parse::<i32>() {
                    Ok(num) => stack.push(Arg::Lit(num)),
                    Err(err) => panic!("{:?}", err),
                },
            }
            cmds = cmds[(idx + 1)..].trim_start().to_string();
        } else {
            return Err("invalid postfix command");
        }
    }
    return Ok(cmds);
}

fn swap(stack: &mut Vec<Arg>) {
    if stack.len() >= 2 {
        let a1 = stack.pop().unwrap();
        let a2 = stack.pop().unwrap();
        stack.push(a1);
        stack.push(a2);
    } else {
        println!("missing arguments on stack");
    }
}

fn exec(stack: &mut Vec<Arg>) {
    if stack.len() >= 1 {
        if let Some(Arg::Seq(cmd)) = stack.pop() {
            match try_run(cmd, stack) {
                Ok(_) => {}
                Err(err) => panic!("{}", err),
            };
        } else {
            println!("mismatched types on stack");
        }
    } else {
        println!("missing arguments on stack");
    }
}
