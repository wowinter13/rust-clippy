#![warn(clippy::unnecessary_reserve)]
#![feature(custom_inner_attributes)]

#[allow(clippy::unnecessary_operation)]
use std::collections::{HashMap, VecDeque};

fn main() {
    vec_reserve();
    vec_deque_reserve();
    hash_map_reserve();
    insufficient_msrv();
    box_vec_reserve();
}

fn vec_reserve() {
    let mut vec: Vec<usize> = vec![];
    let array1: &[usize] = &[1, 2];
    let array2: &[usize] = &[3, 4];

    // do not lint - different arrays
    vec.reserve(array1.len());
    vec.extend(array2);

    // do not lint
    vec.reserve(1);
    vec.extend([1]);

    // do lint
    vec.reserve(array1.len());
    vec.extend(array1);

    // do lint
    {
        vec.reserve(array1.len());
        vec.extend(array1)
    };

    // do not lint
    vec.reserve(array1.len());
    vec.push(1);
    vec.extend(array1);

    // do not lint
    let mut other_vec: Vec<usize> = vec![];
    other_vec.reserve(1);
    other_vec.extend([1]);

    // do not lint
    let mut vec2: Vec<usize> = vec![];
    vec2.extend(array1);
    vec2.reserve(array1.len());
}

fn vec_deque_reserve() {
    let mut vec_deque: VecDeque<usize> = [1].into();
    let array: &[usize] = &[1, 2];

    // do not lint
    vec_deque.reserve(1);
    vec_deque.extend([1]);

    // do lint
    vec_deque.reserve(array.len());
    vec_deque.extend(array);

    // do not lint
    {
        vec_deque.reserve(1);
        vec_deque.extend([1])
    };

    // do not lint
    vec_deque.reserve(array.len() + 1);
    vec_deque.push_back(1);
    vec_deque.extend(array);

    // do not lint
    let mut other_vec_deque: VecDeque<usize> = [1].into();
    other_vec_deque.reserve(1);
    vec_deque.extend([1])
}

fn hash_map_reserve() {
    let mut map: HashMap<usize, usize> = HashMap::new();
    let mut other_map: HashMap<usize, usize> = HashMap::new();
    // do not lint
    map.reserve(other_map.len());
    map.extend(other_map);
}

#[clippy::msrv = "1.61"]
fn insufficient_msrv() {
    let mut vec: Vec<usize> = vec![];
    let array: &[usize] = &[1, 2];

    // do not lint
    vec.reserve(1);
    vec.extend([1]);

    let mut vec_deque: VecDeque<usize> = [1].into();
    let array: &[usize] = &[1, 2];

    // do not lint
    vec_deque.reserve(1);
    vec_deque.extend([1]);
}

fn box_vec_reserve() {
    let mut vec: Box<Vec<usize>> = Box::default();
    let array: &[usize] = &[1, 2];

    // do lint
    vec.reserve(array.len());
    vec.extend(array);
}
