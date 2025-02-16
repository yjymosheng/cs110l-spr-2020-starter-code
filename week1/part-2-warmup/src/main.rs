/* The following exercises were borrowed from Will Crichton's CS 242 Rust lab. */

use std::collections::HashSet;

fn main() {
    println!("Hi! Try running \"cargo test\" to run tests.");
}

fn add_n(v: Vec<i32>, n: i32) -> Vec<i32> {
    v.into_iter().map(|s| s + n).collect()
}

fn add_n_inplace(v: &mut Vec<i32>, n: i32) {
    v.iter_mut().for_each(|s| *s += n);
}

fn dedup(v: &mut Vec<i32>) {
    let mut tmp = Vec::new();

    let mut index = 0;
    loop {
        let ele = v[index];
        if tmp.contains(&ele) {
            v.remove(index);
            index -= 1;
        } else {
            tmp.push(ele);
        }

        index += 1;
        if index == v.len() {
            break;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_n() {
        assert_eq!(add_n(vec![1], 2), vec![3]);
    }

    #[test]
    fn test_add_n_inplace() {
        let mut v = vec![1];
        add_n_inplace(&mut v, 2);
        assert_eq!(v, vec![3]);
    }

    #[test]
    fn test_dedup() {
        let mut v = vec![3, 1, 0, 1, 4, 4];
        dedup(&mut v);
        assert_eq!(v, vec![3, 1, 0, 4]);
    }
}
