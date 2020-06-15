

fn angle(xc: f64, yc: f64, x: f64, y: f64) -> f64 {
    let dx = x - xc;
    let dy = y - yc;

    dy.atan2(dx)
}

fn is_solid(x: f64) -> bool {
    let r = (x.round() - x).abs();
    x.round() > 0.0 && r < 0.0001
}

fn main() {

    let stdin = io::stdin();
    let mut scanner = Scanner::new(stdin.lock());

    let x1: f64 = scanner.next();
    let y1: f64 = scanner.next();
    let x2: f64 = scanner.next();
    let y2: f64 = scanner.next();
    let x3: f64 = scanner.next();
    let y3: f64 = scanner.next();

    let xc = ((x1 * x1 + y1 * y1) * (y2 - y3) + (x2 * x2 + y2 * y2) * (y3 - y1) + (x3 * x3 + y3 * y3) * (y1 - y2)) /
        (   2.0 * ((x1 * (y2 - y3)) - y1 * (x2 - x3) + x2 * y3 - x3 * y2)     );

    let yc = ((x1 * x1 + y1 * y1) * (x3 - x2) + (x2 * x2 + y2 * y2) * (x1 - x3) + (x3 * x3 + y3 * y3) * (x2 - x1)) /
        (   2.0 * ((x1 * (y2 - y3)) - y1 * (x2 - x3) + x2 * y3 - x3 * y2)     );

    let r = ((x1 - xc) * (x1 - xc) + (y1 - yc) * (y1 - yc)).sqrt();

    let a1 = angle(xc, yc, x1, y1);
    let a2 = angle(xc, yc, x2, y2);
    let a3 = angle(xc, yc, x3, y3);

    let da1 = (a1 - a2).abs();
    let da2 = (a2 - a3).abs();
    let da3 = (a3 - a1).abs();

    let mut n = 0;
    let mut a = 0.0;

    for i in 3..101 {
        let d: f64 = 2.0 * PI / i as f64;

        if is_solid(da1 / d) && is_solid(da2 / d) && is_solid(da3 / d) {
            a = d;
            n = i;
            break;
        }
    }

    let s = n as f64 * a.sin() * r * r / 2.0;
    println!("{}", s);
}


////////////////////////////////////////////////////////////////////////////////////////////////////
use std::io;
use std::str::FromStr;
use std::fmt::Debug;

const PI: f64 = std::f64::consts::PI;


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
