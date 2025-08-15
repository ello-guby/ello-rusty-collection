pub fn parse(input: String) -> Vec<String> {
    let mut bank = vec![];
    let mut word = String::new();
    
    let dunk = |
        word: &mut String,
        bank: &mut Vec<String>
    | {
        if !word.is_empty() {
            bank.push(word.to_string());
            word.clear();
        }
    };

    for ch in input.chars() {
        match ch {
            ' ' => {
                dunk(&mut word, &mut bank);
                continue;
            },
            '+' | '-' | '*' | '/' => {
                dunk(&mut word, &mut bank);
                bank.push(ch.to_string());
                continue;
            },
            ch => {
                if ch.is_ascii_digit() || ch == '.' {
                    word.push(ch);
                    continue;
                }
                panic!("{ch} char is invalid");
            },
        }
    }
    dunk(&mut word, &mut bank);
    bank
}

enum MathOp {
    ADD,
    SUB,
    MUL,
    DIV,
}

pub fn calculate_string(math_string: String) -> f64 {
    let math_vec = parse(math_string);
    let mut sum = math_vec.first().unwrap().parse::<f64>().unwrap();
    let mut op = MathOp::MUL;
    let mut first = true;
    for word in math_vec {
        if first { first = false; continue; }
        match word.as_str() {
            "+" => op = MathOp::ADD,
            "-" => op = MathOp::SUB,
            "*" => op = MathOp::MUL,
            "/" => op = MathOp::DIV,
            _ => {
                match word.parse::<f64>() {
                    Ok(val) => {
                        match op {
                            MathOp::ADD => { sum += val; },
                            MathOp::SUB => { sum -= val; },
                            MathOp::MUL => { sum *= val; },
                            MathOp::DIV => { sum /= val; },
                        }
                        op = MathOp::MUL;
                    },
                    Err(e) => panic!("{e}"),
                }
            }
        }
    }
    sum
}
