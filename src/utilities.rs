use std::io;
use std::io::Write;

pub fn get_user_input() -> String{
    print!(">");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failure to read command");

    let user_input = user_input.trim().to_lowercase();

    user_input
}

pub fn split_args_with_quotes(input: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut in_quotes = false;
    let mut current_arg = String::new();

    for c in input.chars() {
        match c {
            ' ' if !in_quotes => {
                if !current_arg.is_empty() {
                    args.push(current_arg.clone());
                    current_arg.clear();
                }
            }
            '"' => {
                in_quotes = !in_quotes;
            }
            _ => {
                current_arg.push(c);
            }
        }
    }

    if !current_arg.is_empty() {
        args.push(current_arg.clone());
    }

    args
}

pub fn generate_text_box(lines: &Vec<String>) -> String{
    let mut max_width = 0;
    for line in lines{
        if line.len() < max_width {
            continue;
        }
        max_width = line.len();
    }
    let box_width = max_width + 6;
    let border = "*".repeat(box_width);

    let mut output = String::new();
    output.push_str(&format!("{}\n",&border));
    for line in lines{
        if line == "&br"{
            output.push_str(&format!("{}\n",&border));
        }else {
            output.push_str(&format!("|  {:<width$}  |\n", line, width = max_width));
        }
    }
    output.push_str(&border);


    output
}