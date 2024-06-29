use std::collections::{HashMap, HashSet};

use safevec::*;

#[test]
fn push() {
    let mut sv = SafeVec::new();

    assert_eq!(0, sv.len());
    assert!(sv.is_empty());

    let g: GenIdx = sv.push("data");

    assert_eq!(Some(&"data"), sv.get(g));
    assert_eq!(1, sv.len());
    assert!(!sv.is_empty());
}

#[test]
fn push_remove() {
    let mut sv = SafeVec::new();

    assert_eq!(0, sv.len());
    assert!(sv.is_empty());

    let g: GenIdx = sv.push("data");

    assert_eq!(Some(&"data"), sv.get(g));
    assert_eq!(1, sv.len());
    assert!(!sv.is_empty());

    assert!(sv.remove(g));

    assert_eq!(None, sv.get(g));
    assert_eq!(0, sv.len());
    assert!(sv.is_empty());
}

#[test]
fn double_remove_item1() {
    let mut sv = SafeVec::new();
    let g1 = sv.push(1);
    let g2 = sv.push(2);
    assert_eq!(Some(&1), sv.get(g1));
    assert_eq!(Some(&2), sv.get(g2));
    assert_eq!(2, sv.len());
    assert!(!sv.is_empty());

    assert!(sv.remove(g1));

    assert_eq!(None, sv.get(g1));
    assert_eq!(Some(&2), sv.get(g2));
    assert_eq!(1, sv.len());
    assert!(!sv.is_empty());

    assert!(!sv.remove(g1));

    assert_eq!(None, sv.get(g1));
    assert_eq!(Some(&2), sv.get(g2));
    assert_eq!(1, sv.len());
    assert!(!sv.is_empty());
}

#[test]
fn double_remove_item2() {
    let mut sv = SafeVec::new();
    let g1 = sv.push(1);
    let g2 = sv.push(2);
    assert_eq!(Some(&1), sv.get(g1));
    assert_eq!(Some(&2), sv.get(g2));
    assert_eq!(2, sv.len());
    assert!(!sv.is_empty());

    assert!(sv.remove(g2));

    assert_eq!(Some(&1), sv.get(g1));
    assert_eq!(None, sv.get(g2));
    assert_eq!(1, sv.len());
    assert!(!sv.is_empty());

    assert!(!sv.remove(g2));

    assert_eq!(Some(&1), sv.get(g1));
    assert_eq!(None, sv.get(g2));
    assert_eq!(1, sv.len());
    assert!(!sv.is_empty());
}

#[test]
fn double_remove_both() {
    let mut sv = SafeVec::new();
    let g1 = sv.push(1);
    let g2 = sv.push(2);
    assert_eq!(Some(&1), sv.get(g1));
    assert_eq!(Some(&2), sv.get(g2));
    assert_eq!(2, sv.len());
    assert!(!sv.is_empty());

    assert!(sv.remove(g1));
    assert!(sv.remove(g2));

    assert_eq!(None, sv.get(g1));
    assert_eq!(None, sv.get(g2));
    assert_eq!(0, sv.len());
    assert!(sv.is_empty());

    assert!(!sv.remove(g1));
    assert!(!sv.remove(g2));

    assert_eq!(None, sv.get(g1));
    assert_eq!(None, sv.get(g2));
    assert_eq!(0, sv.len());
    assert!(sv.is_empty());
}

#[test]
fn iter() {
    let mut sv = SafeVec::new();
    let mut saved = HashMap::new();

    const N: usize = 20;

    let to_remove = [1, 19, 3, 7, 18, 0];

    assert!(N > to_remove.len());

    for x in 0..N {
        saved.insert(x, sv.push(x));
    }

    for x in 0..N {
        assert_eq!(Some(&x), sv.get(saved[&x]));
    }

    assert_eq!(N, sv.len());
    assert!(!sv.is_empty());

    let mut data_in_sv: HashSet<usize> = HashSet::new();

    data_in_sv.extend(sv.iter());

    assert_eq!(HashSet::from_iter(saved.keys().cloned()), data_in_sv);

    for tr in to_remove {
        sv.remove(saved[&tr]);
        saved.remove(&tr);
    }

    assert_eq!(N - to_remove.len(), sv.len());
    assert!(!sv.is_empty());

    data_in_sv.clear();
    data_in_sv.extend(&sv);

    assert_eq!(HashSet::from_iter(saved.keys().cloned()), data_in_sv);
}

#[test]
fn iter_mut() {
    let mut sv = SafeVec::new();
    let mut saved_before = HashMap::new();
    let mut saved_after = HashMap::new();

    const N: usize = 20;

    let mut to_remove = [1, 19, 3, 7, 18, 0];

    assert!(N > to_remove.len());

    for x in 0..N {
        let g = sv.push(x);
        saved_before.insert(x, g);
        saved_after.insert(x * 42, g);
    }

    for x in 0..N {
        assert_eq!(Some(&x), sv.get(saved_before[&x]));
    }

    assert_eq!(N, sv.len());
    assert!(!sv.is_empty());

    sv.iter_mut().for_each(|x| *x *= 42);

    let mut data_in_sv: HashSet<usize> = HashSet::new();

    data_in_sv.extend(sv.iter());

    assert_eq!(HashSet::from_iter(saved_after.keys().cloned()), data_in_sv);

    to_remove.iter_mut().for_each(|x| *x *= 42);

    for tr in to_remove {
        sv.remove(saved_after[&tr]);
        saved_after.remove(&tr);
    }

    assert_eq!(N - to_remove.len(), sv.len());
    assert!(!sv.is_empty());

    data_in_sv.clear();
    data_in_sv.extend(&sv);

    assert_eq!(HashSet::from_iter(saved_after.keys().cloned()), data_in_sv);
}
