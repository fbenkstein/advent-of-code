fn toggle_case(c: char) -> String {
    let upper: String = c.to_uppercase().collect();
    if c.to_string() == upper {
        c.to_lowercase().collect()
    } else {
        upper
    }
}

fn react(input: &str) -> usize {
    let mut stack: String = String::new();
    input.chars().for_each(|c| {
        if !stack.is_empty() && c.to_string() == toggle_case(stack.chars().last().unwrap()) {
            stack.pop();
        } else {
            stack.push(c);
        }
    });
    stack.len()
}

pub fn solve(input: &str) -> usize {
    char_iter::new('a', 'z')
        .map(|c| {
            react(
                &input
                    .replace(c, "")
                    .replace(&c.to_uppercase().collect::<String>(), ""),
            )
        }).min()
        .unwrap()
}
