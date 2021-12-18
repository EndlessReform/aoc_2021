use crate::Error;
use std::collections::HashMap;

fn power_consumption(s: &str) -> Result<(), Error> {
    let mut freqs = HashMap::new();
    for line in s.split_terminator('\n') {
        for (i, byte) in line.bytes().enumerate() {
            let amt = match byte {
                48 => -1,
                49 => 1,
                _ => return Err(Error::Parse),
            };
            let count = freqs.entry(i).or_insert(0);
            *count += amt;
        }
    }
    let mut gamma = 0;
    let mut epsilon = 0;

    // We have to do this because freqs isn't ordered
    for i in 0..freqs.len() {
        let is_1: bool;
        match freqs.get(&i) {
            Some(n) => is_1 = n > &0,
            _ => return Err(Error::Parse),
        }

        gamma <<= 1;
        epsilon <<= 1;
        if is_1 {
            gamma += 1
        } else {
            epsilon += 1
        };
    }

    println!("Power consumption: {}", gamma * epsilon);
    Ok(())
}

fn get_rating(biggest: bool, input: &Vec<&str>, l: usize) -> Result<i32, Error> {
    let mut ns = input.to_owned();

    for char in 0..l {
        let mut weight = 0;
        // Get most common value at that bit
        for n in &ns {
            match n.chars().nth(char) {
                Some('1') => weight += 1,
                Some('0') => weight -= 1,
                _ => return Err(Error::Parse),
            }
        }

        let lead_char = if weight >= 0 { '1' } else { '0' };
        ns = ns
            .iter()
            .filter(|g| (g.chars().nth(char) == Some(lead_char)) ^ !biggest)
            .copied()
            .collect();

        // println!("{:?}", ns);

        if ns.len() == 1 {
            // Guaranteed never to fail
            let x = i32::from_str_radix(ns[0], 2)?;
            return Ok(x);
        }
    }
    // We don't know WTF happened; return
    Err(Error::Parse)
}

fn life_support(s: &str) -> Result<(), Error> {
    let nums: Vec<&str> = s.split_terminator('\n').collect();

    let l = nums[0].len();
    let oxygen_rating = get_rating(true, &nums, l)?;
    let scrubber_rating = get_rating(false, &nums, l)?;



    println!("{}", oxygen_rating * scrubber_rating);
    /*
        let mut s_weight = 0;
        for s in &mut scrub_nums {
            match s.chars().nth(0) {
                Some('1') => s_weight += 1,
                Some('0') => s_weight -= 1,
                _ => return Err(Error::Parse)
            }
        }

    */
    Ok(())
}

pub fn day3(s: String) -> Result<(), Error> {
    if let Err(x) = power_consumption(&s) {
        return Err(x);
    }
    life_support(&s)
}
