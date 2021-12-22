use std::error::Error;

/*
Time to respawn: zero-index
Day 0: 3
Day 2: 0
Day 3: Reset, spawn new

Doubles every 7 days
On first time: takes extra

Input: ages, (ndays)
Output: sum
*/
fn parse_pop(s: &str) -> Result<Vec<i64>, Box<dyn Error>> {
    let mut res = vec![0; 9];

    // Invariant: values <= 8
    let fish = s.split(',');
    for f in fish {
        let i = f.parse::<i64>()?;
        // Ignore bad numbers for now; this is a toy problem
        if i < 9 {
            res[i as usize] += 1;
        }
    }
    Ok(res)
}

pub fn day6(s: &str) -> Result<(), Box<dyn Error>> {
    // Maybe make this a parameter?
    let ndays = 256;

    let mut pop = parse_pop(s)?;
    // println!("{:?}", pop);

    for _ in 0..ndays {
        // These indices will never fail
        let n_0 = pop[0];
        // Shift all other fish 1 day earlier
        pop = pop[1..].to_vec();
        // All new fish now have 8 days
        pop.push(n_0);
        // Reset timer for original fish
        pop[6] += n_0;
    }
    let nfish = pop.iter().fold(0, |acc, x| acc + x);
    println!("{}", nfish);
    Ok(())
}
