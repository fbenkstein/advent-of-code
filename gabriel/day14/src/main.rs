use std::fmt;

#[derive(Debug)]
struct HotChocolateFactory {
    recipes: Vec<u8>,
    current_recipes: (usize, usize),
}

impl Default for HotChocolateFactory {
    fn default() -> Self {
        Self {
            recipes: vec![3, 7],
            current_recipes: (0, 1),
        }
    }
}

impl HotChocolateFactory {
    pub fn next_recipe(&mut self) {
        let combined_recipe =
            self.recipes[self.current_recipes.0] + self.recipes[self.current_recipes.1];
        self.recipes.extend(Self::digits_to_vec(combined_recipe));
        let next_current_recipe = |&e| (e + 1 + self.recipes[e] as usize) % self.recipes.len();
        self.current_recipes = (
            next_current_recipe(&self.current_recipes.0),
            next_current_recipe(&self.current_recipes.1),
        );
    }

    fn digits_to_vec(n: u8) -> Vec<u8> {
        let mut digits = Vec::new();
        let mut n = n;
        while n > 9 {
            digits.push(n % 10);
            n = n / 10;
        }
        digits.push(n);
        digits.reverse();
        digits
    }

    fn elves_training(&mut self, train_for: usize) -> String {
        for _ in 0..train_for + 10 {
            self.next_recipe();
        }
        self.recipes[train_for..train_for + 10]
            .iter()
            .map(|i| i.to_string())
            .collect::<String>()
    }

    fn gniniart_sevle(&mut self, recipes: &[u8]) -> usize {
        loop {
            self.next_recipe();
            if self.recipes.len() > recipes.len() {
                if self.recipes[self.recipes.len() - recipes.len()..self.recipes.len()] == *recipes
                {
                    return self.recipes.len() - recipes.len();
                }
            }
        }
    }
}

impl fmt::Display for HotChocolateFactory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.recipes
                .iter()
                .enumerate()
                .map(|(idx, recipe)| if idx == self.current_recipes.0 {
                    format!("({})", recipe)
                } else if idx == self.current_recipes.1 {
                    format!("[{}]", recipe)
                } else {
                    format!(" {} ", recipe)
                })
                .collect::<String>()
                .trim_end(),
        )
    }
}

fn main() {
    let mut factory = HotChocolateFactory::default();
    println!("{}", factory.elves_training(030121));
    let mut factory = HotChocolateFactory::default();
    println!("{}", factory.gniniart_sevle(&[0, 3, 0, 1, 2, 1]))
}

#[test]
fn example_test() {
    let input = r#"(3)[7]
(3)[7] 1  0
 3  7  1 [0](1) 0
 3  7  1  0 [1] 0 (1)
(3) 7  1  0  1  0 [1] 2
 3  7  1  0 (1) 0  1  2 [4]
 3  7  1 [0] 1  0 (1) 2  4  5
 3  7  1  0 [1] 0  1  2 (4) 5  1
 3 (7) 1  0  1  0 [1] 2  4  5  1  5
 3  7  1  0  1  0  1  2 [4](5) 1  5  8
 3 (7) 1  0  1  0  1  2  4  5  1  5  8 [9]
 3  7  1  0  1  0  1 [2] 4 (5) 1  5  8  9  1  6
 3  7  1  0  1  0  1  2  4  5 [1] 5  8  9  1 (6) 7
 3  7  1  0 (1) 0  1  2  4  5  1  5 [8] 9  1  6  7  7
 3  7 [1] 0  1  0 (1) 2  4  5  1  5  8  9  1  6  7  7  9
 3  7  1  0 [1] 0  1  2 (4) 5  1  5  8  9  1  6  7  7  9  2
"#;

    let mut factory = HotChocolateFactory::default();
    for line in input.lines() {
        assert_eq!(line, format!("{}", factory));
        factory.next_recipe();
    }
}

#[test]
fn training_test() {
    let mut factory = HotChocolateFactory::default();
    assert_eq!(factory.elves_training(5), "0124515891");

    let mut factory = HotChocolateFactory::default();
    assert_eq!(factory.elves_training(9), "5158916779");

    let mut factory = HotChocolateFactory::default();
    assert_eq!(factory.elves_training(18), "9251071085");

    let mut factory = HotChocolateFactory::default();
    assert_eq!(factory.elves_training(2018), "5941429882");
}

#[test]
fn backwards_training_test() {
    let mut factory = HotChocolateFactory::default();
    assert_eq!(factory.gniniart_sevle(&[5, 1, 5, 8, 9]), 9);

    let mut factory = HotChocolateFactory::default();
    assert_eq!(factory.gniniart_sevle(&[0, 1, 2, 4, 5]), 5);

    let mut factory = HotChocolateFactory::default();
    assert_eq!(factory.gniniart_sevle(&[9, 2, 5, 1, 0]), 18);

    let mut factory = HotChocolateFactory::default();
    assert_eq!(factory.gniniart_sevle(&[5, 9, 4, 1, 4]), 2018);
}
