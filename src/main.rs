use std::collections::HashMap;
use std::hash::Hash;

fn main() {
    let v: Vec<(i32, i32)> = vec![(2, 3), (1, 1), (2, 3)];
    let s = [(1, 1), (2, 3), (2, 3)];

    // Compare v and s for equality.
    println!("{}", comprises(&v, &s));

    let w: Vec<(i32, i32)> = vec![(2, 3), (2, 3), (1, 1)];

    // Compare v and w for equality.
    println!("{}", comprises(&v, &w));
    
    // Compare w and s for equality
    println!("{}", comprises(&w, &s));
}

fn comprises<'a, V, S, T: 'a>(v: V, s: S) -> bool
    where V: IntoIterator<Item=&'a T>,
          S: IntoIterator<Item=&'a T>,
          T: Eq + Hash + Copy,
{
    // Construct a bag of the contents of v
    let mut vb: HashMap<T, usize> = HashMap::new();
    for i in v {
        bump(&mut vb, &i);
    }
    // Desugared version
//    match IntoIterator::into_iter(v) {
//        mut iter => loop {
//            match iter.next() {
//                Some(x) => { bump(&mut vb, &x); },
//                None => break,
//            }
//        },
//    };

    // Construct a bag of the contents of s
    let mut sb: HashMap<T, usize> = HashMap::new();
    for i in s {
        bump(&mut sb, &i);
    }

    // Compare the bags
    vb == sb
}

// Bump the ith entry of bag b
fn bump<T>(b: &mut HashMap<T, usize>, i: &T)
    where T: Eq + Hash + Copy
{
    let count = b.entry(*i).or_insert(0);
    *count += 1;
}
