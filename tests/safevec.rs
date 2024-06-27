use std::collections::HashSet;

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
    let g1 = sv.push(1);
    let g2 = sv.push(2);
    assert_eq!(Some(&1), sv.get(g1));
    assert_eq!(Some(&2), sv.get(g2));
    assert_eq!(2, sv.len());
    assert!(!sv.is_empty());

    let mut data_in_sv = HashSet::new();

    for d in &sv {
        data_in_sv.insert(*d);
    }

    assert_eq!(HashSet::from([1, 2]), data_in_sv);
}

#[test]
fn iter_mut() {
    let mut sv = SafeVec::new();
    let g1 = sv.push(1);
    let g2 = sv.push(2);
    assert_eq!(Some(&1), sv.get(g1));
    assert_eq!(Some(&2), sv.get(g2));
    assert_eq!(2, sv.len());
    assert!(!sv.is_empty());

    for d in &mut sv {
        *d *= 42;
    }

    let mut data_in_sv = HashSet::new();

    for d in &sv {
        data_in_sv.insert(*d);
    }

    assert_eq!(HashSet::from([42, 84]), data_in_sv);
}
