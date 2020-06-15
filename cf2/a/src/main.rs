

fn main() {
    let stdin = io::stdin();
    let mut scanner = Scanner::new(stdin.lock());

    let n: usize  = scanner.next();

    let mut names: HashMap<String, i32> = HashMap::new();
    let mut scores: Vec<(String, i32)> = Vec::new();

    for _ in 0..n {
        let name: String = scanner.next();
        let value: i32 = scanner.next();

        if let Some(&val) = names.get(&name) {
            names.insert(name.clone(), value + val);
            scores.push((name, value + val));
        } else {
            names.insert(name.clone(), value);
            scores.push((name, value));
        }
    }

    let mut cur_best = 0;

    for (_, s) in &names {
        if cur_best < *s {
            cur_best = *s;
        }
    }

    let mut bnames: HashSet<String> = HashSet::new();

    for (n, s) in names {
        if cur_best == s {
            bnames.insert(n);
        }
    }

    if bnames.len() > 1 {

        for (name, score) in scores.iter() {

            if !bnames.contains(name) {
                continue;
            }

            // println!("check {} score {}", s, score);

            if *score >= cur_best {
                println!("{}", name);
                break;
            }
        }

    } else {
        println!("{}", &bnames.iter().next().unwrap());
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
use std::io;
use std::str::FromStr;
use std::fmt::Debug;
use std::collections::{HashMap, HashSet};

pub struct Scanner<B> {
    reader: B,
    buffer: Vec<String>
}

impl <B: io::BufRead> Scanner<B> {
    pub fn new(reader: B) -> Self {
        Self {
            reader,
            buffer: Vec::new()
        }
    }

    pub fn next<T: FromStr>(&mut self) -> T
        where T::Err: Debug {

        if let Some(front) = self.buffer.pop() {
            front.parse::<T>().expect(&front)
        } else {
            let mut input = String::new();
            self.reader.read_line(&mut input).expect("line not read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
            self.next()
        }
    }
}

