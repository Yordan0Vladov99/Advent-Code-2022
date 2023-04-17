use std::{fs::File, io::Read};

fn main() {
    let mut path = File::open("input.txt").unwrap();
    let mut input = String::new();
    path.read_to_string(&mut input).unwrap();
    let lines = input.trim().split('\n').collect::<Vec<_>>();

    let mut total = 0;

    for line in lines.iter() {
        let mut coef: i64 = 1;
        for c in line.chars().rev() {
            total += ("=-012".to_string().find(c).unwrap() as i64 - 2) * coef;
            println!("{}", coef);
            coef *= 5;
        }
    }

    let mut output = String::new();

    while total > 0 {
        let rem = total % 5;
        total /= 5;

        if rem <= 2 {
            let mut rem_str = rem.to_string();
            rem_str.push_str(&output);
            output = rem_str;
        } else {
            let mut new_output = String::new();
            new_output.push(*"   =-".as_bytes().get(rem as usize).unwrap() as char);
            new_output.push_str(&output);
            output = new_output;
            total += 1;
        }
    }
    println!("{}", output)
}
