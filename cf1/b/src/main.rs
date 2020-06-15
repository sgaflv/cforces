use std::io;


fn main() {
    let mut line= String::new();
    io::stdin().read_line(&mut line).unwrap();

    let n: i32 = line.trim().parse().expect("failed to parse the first parameter");

    for _ in 0..n {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        let line = line.trim().as_bytes();

        let mut letters_coord: i32 = 0;
        let mut digits_coord: i32 = 0;
        let mut digits_coord2: i32 = 0;
        let mut found_digit = false;
        let mut found_letter_after_digit = false;

        for c in line {
            let is_letter = (b'A'..b'Z'+1).contains(c);
            let is_digit = (b'0'..b'9'+1).contains(c);

            if found_digit {
                if is_letter {
                    found_letter_after_digit = true
                }
            }

            if is_digit {
                found_digit = true;
            }

            if is_letter {
                letters_coord = letters_coord * 26 + (*c as i32) - (b'A' as i32) + 1;
            }

            if is_digit {
                if !found_letter_after_digit {
                    digits_coord = digits_coord * 10 + (*c as i32) - (b'0' as i32);
                } else {
                    digits_coord2 = digits_coord2 * 10 + (*c as i32) - (b'0' as i32);
                }
            }

        }

        letters_coord = letters_coord;


        if !found_letter_after_digit {
            println!("{}", rc_coord(letters_coord, digits_coord))
        } else {
            println!("{}", a1_coord(digits_coord2, digits_coord))
        }

    }
}

fn rc_coord(x: i32, y: i32) -> String {
    format!("R{}C{}", y, x)
}

fn coord_to_letters(x: i32) -> String {
    let mut letters = String::new();
    let mut x = x;

    while x > 0 {

        let mut rem = (x % 26) as u8;

        if rem == 0 {
            rem = 26;
            x = x - 26;
        }

        letters.push(((rem + (b'A' - 1)) as u8) as char);
        x = x / 26;
    }

    let letters: String = letters.chars().rev().collect();

    letters
}

fn a1_coord(x: i32, y: i32) -> String {
    format!("{}{}", coord_to_letters(x), y)
}
