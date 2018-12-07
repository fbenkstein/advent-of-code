use std::borrow::Borrow;

fn should_eliminate<C1: Borrow<char>, C2: Borrow<char>>(c1: C1, c2: C2) -> bool {
    let c1 = c1.borrow();
    let c2 = c2.borrow();
    return c1.is_uppercase() != c2.is_uppercase()
        && c1.to_uppercase().to_string() == c2.to_uppercase().to_string();
}

fn main() {
    let data = include_str!("input_05.txt");
    let mut output = Vec::with_capacity(data.len());

    for c in data.chars().filter(|c| c.is_alphabetic()) {
        let maybe_last_c = output.last().cloned();

        match maybe_last_c {
            Some(last_c) if should_eliminate(last_c, c) => {
                output.pop();
            }
            _ => output.push(c),
        }
    }

    println!("{}", output.len());
}
