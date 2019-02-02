use std::borrow::Borrow;

trait PolymerUnit {
    fn reacts_with<T: Borrow<Self>>(&self, other: T) -> bool;
}

impl PolymerUnit for char {
    fn reacts_with<T: Borrow<Self>>(&self, other: T) -> bool {
        let other = other.borrow();
        return self.is_uppercase() != other.is_uppercase()
            && self.to_uppercase().eq(other.to_uppercase());
    }
}

trait Polymer<T>: Iterator<Item = T>
where
    T: PolymerUnit + Clone,
{
    fn react(&mut self) -> Vec<Self::Item> {
        let mut result = Vec::<T>::new();

        while let Some(ref unit) = self.next() {
            let maybe_last_unit = result.last();

            match maybe_last_unit {
                Some(last_unit) if last_unit.reacts_with(unit) => {
                    result.pop();
                }
                _ => result.push(unit.clone()),
            }
        }

        result
    }
}

impl<T, U> Polymer<U> for T
where
    T: Iterator<Item = U>,
    U: PolymerUnit + Clone,
{
}

fn main() {
    let input = include_str!("input_05.txt")
        .chars()
        .filter(|c| c.is_alphabetic());
    (b'a'..=b'z')
        .map(|b| b as char)
        .map(|filtered_c| {
            input
                .clone()
                .filter(move |c| !c.to_uppercase().eq(filtered_c.to_uppercase()))
        })
        .map(move |mut filtered_input| filtered_input.react())
        .map(|output| output.len())
        .min()
        .map(|x| println!("{}", x));
}
