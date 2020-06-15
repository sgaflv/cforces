
fn p2(a: f64) -> f64 { return a * a; }

fn dist(xp: f64, yp: f64, x: &Vec<f64>, y: &Vec<f64>, r: &Vec<f64>) -> f64 {
    let a=(p2(xp - x[0]) + p2(yp - y[0])).sqrt() / r[0];
    let b=(p2(xp - x[1]) + p2(yp - y[1])).sqrt() / r[1];
    let c=(p2(xp - x[2]) + p2(yp - y[2])).sqrt() / r[2];
    return p2(a - b) + p2(a - c) + p2(b - c);
}

fn eq(a: f64, b: f64) -> bool{
    return (a-b).abs() < 0.000000001;
}

fn main() {

    let stdin = io::stdin();
    let mut scanner = Scanner::new(stdin.lock());

    let mut x = vec![0f64; 3];
    let mut y = vec![0f64; 3];
    let mut r = vec![0f64; 3];

    for i in 0..3 {
        x[i] = scanner.next();
        y[i] = scanner.next();
        r[i] = scanner.next();
    }

    let mut xp = (x[0] + x[1] + x[2]) / 3.0;
    let mut yp = (y[0] + y[1] + y[2]) / 3.0;

    let mut d = dist(xp, yp, &x, &y, &r);

    let mut step = 100f64;

    while step > 0.0000009 {

        let d1 = dist(xp - step, yp, &x, &y, &r);
        let d2 = dist(xp + step, yp, &x, &y, &r);
        let d3 = dist(xp, yp - step, &x, &y, &r);
        let d4 = dist(xp, yp + step, &x, &y, &r);

        if      d1 < d && !eq(d1,d)  {
            xp -= step;
        } else if d2 < d && !eq(d2,d) {
            xp += step;
        } else if d3 < d && !eq(d3,d) {
            yp -= step;
        } else if d4 < d && !eq(d4,d) {
            yp += step;
        } else {
            step *= 0.1;
        }

        d = dist(xp, yp, &x, &y, &r);
    }

    if d < 0.0001 {
        print!("{} {}",xp, yp);
    }

    println!();
}

////////////////////////////////////////////////////////////////////////////////////////////////////
use std::io;
use std::str;

pub struct Scanner<R> {
    reader: R,
    buf_str: Vec<u8>,
    buf_iter: str::SplitAsciiWhitespace<'static>,
}

impl<R: io::BufRead> Scanner<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buf_str: vec![],
            buf_iter: "".split_ascii_whitespace(),
        }
    }

    pub fn next<T: str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok().expect("Failed parse");
            }
            self.buf_str.clear();
            self.reader
                .read_until(b'\n', &mut self.buf_str)
                .expect("Failed read");
            self.buf_iter = unsafe {
                let slice = str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_ascii_whitespace())
            }
        }
    }
}