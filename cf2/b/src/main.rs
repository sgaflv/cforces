fn count_points(mut num: i32, div: i32) -> i32 {
    let mut sum = 0i32;

    while num > 0 && num % div == 0 {

        sum += 1;
        num /= div;
    }

    sum
}

fn solve(n: usize, icost: &Vec<Vec<i32>>, div: i32) -> (i32, Vec<u8>) {

    let mut res_vec: Vec<u8> = Vec::with_capacity(2 * n);

    let mut dir: Vec<Vec<u8>> = vec![vec![0; n]; n];
    let mut cost: Vec<Vec<i32>> = vec![vec![0; n]; n];
    let mut scores: Vec<Vec<i32>> = vec![vec![0; n]; n];

    for y in 0..n {
        for x in 0..n {
            cost[y][x] = count_points(icost[y][x], div);
        }
    }

    for y in (0..n).rev() {
        for x in (0..n).rev() {

            if x == n-1 && y == n-1 {
                scores[y][x] = cost[y][x];
                continue;
            } else if x == n-1 {
                dir[y][x] = b'D';
            } else if y == n-1 {
                dir[y][x] = b'R';
            } else {
                let cost_down = cost[y][x] + scores[y+1][x];
                let cost_right = cost[y][x] + scores[y][x+1];

                if cost_down > cost_right {
                    dir[y][x] = b'R';
                } else {
                    dir[y][x] = b'D';
                }
            }

            if dir[y][x] == b'D' {
                scores[y][x] = cost[y][x] + scores[y+1][x];
            } else {
                scores[y][x] = cost[y][x] + scores[y][x+1];
            }
        }
    }

    let mut x = 0;
    let mut y = 0;

    let mut score2: i32 = 0;
    let mut score5: i32 = 0;

    score2 += count_points(icost[0][0], 2);
    score5 += count_points(icost[0][0], 5);

    while x < n-1 || y < n-1 {
        res_vec.push(dir[y][x]);

        if dir[y][x] == b'D' {
            y+=1;
        } else {
            x+=1;
        }

        score2 += count_points(icost[y][x], 2);
        score5 += count_points(icost[y][x], 5);
    }

    //println!("{:?} {} {} {}", cost, String::from_utf8(res_vec.clone()).unwrap(), score2, score5);

    return (score2.min(score5), res_vec);
}

fn main() {
    let stdin = io::stdin();
    let mut scanner = Scanner::new(stdin.lock());

    let n: usize = scanner.next();

    let mut cost: Vec<Vec<i32>> = vec![vec![0; n]; n];

    let mut zero_found = false;
    let mut zero_y = 0usize;

    for y in 0..n {
        for x in 0..n {
            cost[y][x] = scanner.next();
            if cost[y][x] == 0 {
                zero_found = true;
                zero_y = y;
            }
        }
    }

    let (score2, dir2) = solve(n, &cost, 2);
    let (score5, dir5) = solve(n, &cost, 5);

    let mut score = score2;
    let mut dir = dir2;
    if score5 < score2 {
        score = score5;
        dir = dir5;
    }

    if score > 1 && zero_found {
        println!("1");
        for _ in 0..zero_y {
            print!("D");
        }
        for _ in 0..n-1 {
            print!("R");
        }
        for _ in zero_y..n-1 {
            print!("D");
        }

        return;
    }

    println!("{}", score);
    for b in dir {
        print!("{}", b as char);
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