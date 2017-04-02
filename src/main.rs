use std::collections::HashMap;
use std::hash::Hash;

fn main() {
    let v = vec![(2,3), (1,1), (2,3)];
    let xs: [(i32,i32); 3] = [(1,1), (2,3), (2,3)];
    let s = &xs;

    // Compare v and s for equality.

    println!("{}", comprises(v, s));
}


fn comprises<T>(v: Vec<T>, s: &[T]) -> bool
    where T: Eq + Hash + Copy {

    // Construct a bag of the contents of v
    let mut vb: HashMap<T, u32> = HashMap::new();
    for i in &v {
        bump(&mut vb, &i);
    }

    // Construct a bag of the contents of s
    let mut sb: HashMap<T, u32> = HashMap::new();
    for i in s {
        bump(&mut sb, &i);
    }

    // Compare the bags
    vb == sb
}

// Bump the ith entry of bag b
fn bump<T>(b: &mut HashMap<T, u32>, i: &T)
    where T: Eq + Hash + Copy {
    let count = b.entry(*i).or_insert(0);
    *count += 1;
}
