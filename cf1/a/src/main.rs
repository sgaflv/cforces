
fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut scanner = Scanner::new(stdin.lock());

    let n: u64 = scanner.next();


    let m: u64 = scanner.next();
    let a: u64 = scanner.next();

    let x = (n + a - 1) / a;
    let y = (m + a - 1) / a;

    println!("{}", x * y);

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
use std::io;
use std::str::FromStr;
use std::fmt::Debug;

pub struct Scanner<B> {
    reader: B,
    buffer: Vec<String>
}

impl <B: io::BufRead> Scanner<B> {
    pub fn new(reader: B) -> Self {
        Self {
            reader: reader,
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