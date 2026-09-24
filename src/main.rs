fn main() {
    let mut string = String::from("MATMEX LUCHSHE VSEH");
    println!("{}", process(&mut string));
}

fn process(text: &mut String) -> &str {
    text.push('!');
    let trimmed = text.trim();
    match trimmed.split_whitespace().next() {
        Some(val) => {
            if val.len() == trimmed.len() {
                &val[0..val.len() - 1]
            } else {
                val
            }
        }
        None => &text[0..0],
    }
}

#[cfg(test)]
#[path = "../tests/unit/process.rs"]
mod process_tests;
