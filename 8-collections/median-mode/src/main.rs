use std::collections::HashMap;

fn main() {
    let mut v1: Vec<i32> = vec![];
    let mut v2 = vec![1];
    let mut v3 = vec![1, 2];
    let mut v4 = vec![1, 2, 3];
    let mut v5 = vec![1, 2, 3, 4];

    match median(&mut v1) {
        None => println!("v1 has no median"),
        Some(num) => println!("v1 median is {num}")
    }
    match median(&mut v2) {
        None => println!("v2 has no median"),
        Some(num) => println!("v2 median is {num}")
    }
    match median(&mut v3) {
        None => println!("v3 has no median"),
        Some(num) => println!("v3 median is {num}")
    }
    match median(&mut v4) {
        None => println!("v4 has no median"),
        Some(num) => println!("v4 median is {num}")
    }
    match median(&mut v5) {
        None => println!("v5 has no median"),
        Some(num) => println!("v5 median is {num}")
    }

    v2 = vec![1];
    v3 = vec![1, 1];
    v4 = vec![1, 2, 3, 5, 6, 3, 6, 6];
    v5 = vec![1, 2, 3, 4, 2];

    match mode(&mut v1) {
        None => println!("v1 has no mode"),
        Some(num) => println!("v1 mode is {num}")
    }
    match mode(&mut v2) {
        None => println!("v2 has no mode"),
        Some(num) => println!("v2 mode is {num}")
    }
    match mode(&mut v3) {
        None => println!("v3 has no mode"),
        Some(num) => println!("v3 mode is {num}")
    }
    match mode(&mut v4) {
        None => println!("v4 has no mode"),
        Some(num) => println!("v4 mode is {num}")
    }
    match mode(&mut v5) {
        None => println!("v5 has no mode"),
        Some(num) => println!("v5 mode is {num}")
    }
}

fn median(v: &mut Vec<i32>) -> Option<f64> {
    v.sort();

    let len = v.len();
    if len == 0 {
        return None;
    }

    let index = len / 2;
    match v.len() % 2 == 0 {
        true => {
            return Some((v[index] + v[index - 1]) as f64 / 2.0);
        }
        false => return Some(v[index] as f64)
    }
}

fn mode(v: &Vec<i32>) -> Option<i32> {
    let mut freqMap: HashMap<i32, i32> = HashMap::new();
    let mut mode: Option<i32> = None;
    let mut freq = 0;

    for x in v {
        let count = freqMap.entry(*x).or_insert(0);
        *count += 1;

        if *count > freq {
            freq = *count;
            mode = Some(*x);
        }
    };

    return mode;
}

