// Solution attempt with max tropical semiring and adjacence matrix approach.
//
// Non-working code. Kept for archive reasons.

use ndarray::Array2;
use num_traits::{One, Zero};

// Max tropical semiring with underlying set ℝ ∪ {-∞}.
#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
struct Entry(Option<u32>); // None represents -∞

impl fmt::Debug for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(value) = self.0 {
            write!(f, "{}", value)
        } else {
            f.write_str("-∞")
        }
    }
}

impl Zero for Entry {
    fn zero() -> Self {
        Self(None)
    }

    fn is_zero(&self) -> bool {
        self.0.is_none()
    }
}

impl One for Entry {
    fn one() -> Self {
        Self(Some(0))
    }
}

impl ops::Add for Entry {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(self.0.max(other.0)) // None is smaller than any other element
    }
}

impl ops::Sub for Entry {
    type Output = Self;
    fn sub(self, _other: Self) -> Self {
        unimplemented!(); // no subtraction in semirings
    }
}

impl ops::Mul for Entry {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self(self.0.and_then(|a| other.0.map(|b| a + b)))
    }
}

impl ops::Div for Entry {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Self(self.0.and_then(|a| other.0.map(|b| a - b)))
    }
}

fn index_to_vertex(index: usize) -> char {
    (index as u8 + 'A' as u8) as char
}

fn order(edges: &[(usize, usize)]) -> String {
    let num_vertices = edges.iter().map(|&(from, to)| from.max(to)).max().unwrap() + 1;

    // adjacence matrix of the graph
    let mut adj_matrix: Array2<Entry> = Array2::eye(num_vertices); // identity matrix
    for &(from, to) in edges {
        adj_matrix[[from, to]] = Entry(Some(1));
    }

    let longest_path_matrix =
        (0..edges.len()).fold(adj_matrix.clone(), |mat, _| mat.dot(&adj_matrix));

    // add lexicographic priorities to adjacence matrix based on the longest_path matrix
    // let mut added_edges = 0;
    // for v in 0..num_vertices - 1 {
    //     let w = (v + 1..num_vertices).find(|&w| longest_path_matrix[[w, v]].is_zero());
    //     if let Some(w) = w {
    //         println!("{} -> {}", index_to_vertex(v), index_to_vertex(w));
    //         adj_matrix[[v, w]] = Entry(Some(1));
    //         added_edges += 1;
    //     }
    // }
    // println!("{:?}", adj_matrix);

    // for (v, w) in (0..num_vertices - 1).zip(1..num_vertices) {
    //     if longest_path_matrix[[w, v]].is_zero() {
    //         // there is no path from w to v
    //         adj_matrix[[v, w]] = Entry(Some(1));
    //         added_edges += 1;
    //     }
    // }

    // recompute longest path matrix with added lexicographical priorities
    // let longest_path_matrix =
    //     (0..edges.len() + added_edges).fold(adj_matrix.clone(), |mat, _| mat.dot(&adj_matrix));

    println!("{:?}", longest_path_matrix);

    let longest_outgoing_paths = longest_path_matrix
        .outer_iter()
        .enumerate()
        .map(|(idx, row)| {
            let vertex = index_to_vertex(idx);
            let longest_path = *row.iter().max().unwrap();
            (longest_path, vertex)
        })
        .sorted_by(|(path_a, a), (path_b, b)| path_b.cmp(path_a).then(a.cmp(b)));

    longest_outgoing_paths
        .iter()
        .map(|(_, vertex)| vertex)
        .collect()
}
