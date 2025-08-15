use std::env::args;
use ello_rusty_collection::math::{
    parse as math_parse,
    calculate_string
};

fn main() {
    // "n0" mean no argv0 which is the name of the running file.
    let arg_strn0: String;
    {
        let mut first = true;
        let mut arg_vecn0 = vec![];
        for arg in args() {
            if first { first = false; continue; }
            arg_vecn0.push(arg);
        }
        arg_strn0 = arg_vecn0.join(" ");
    }
    let result = calculate_string(arg_strn0.to_string());
    let equation = format!("{} = {result}", math_parse(arg_strn0).join(" "));
    println!("{equation}"); 
}
