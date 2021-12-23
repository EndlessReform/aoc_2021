use std::error::Error;

fn parse_arr(s: &str) -> Result<Vec<i32>,Box<dyn Error>> {
    let mut res = Vec::new();
    for raw_n in s.split(',') {
        res.push(raw_n.parse::<i32>()?);
    }
    Ok(res)
}

fn median(us: &Vec<i32>) -> i32 {
    if us.is_empty() {
        // Perverse!
        return -1;
    }
    let mut v = us.to_owned();
    v.sort_unstable();
    v[v.len() / 2]
}

fn get_cost(align_pos: i32, locs: &Vec<i32>) -> i32 {
    locs.iter().fold(0, |acc, x| acc + (x - align_pos).abs())
}

fn get_triangular_cost(align_pos: i32, locs: &Vec<i32>) -> i32 {
    fn triangle(x: i32) -> i32 {
        x * (x + 1) / 2
    }
    locs.iter().fold(0, |acc, x| acc + triangle((x - align_pos).abs()))
}

// I got bored today
fn brute_force(locs: &Vec<i32>) -> i32 {
    let mut sorted = locs.to_owned();
    sorted.sort();
    let min = sorted[0];
    let max = sorted[sorted.len() - 1];

    let mut lowest_cost: i32 = i32::MAX;
    for i in min..max {
        let curr_cost = get_triangular_cost(i, &sorted);
        if curr_cost < lowest_cost {
            lowest_cost = curr_cost;
        }
    }
    lowest_cost
}

pub fn day7(s: &str) -> Result<(), Box<dyn Error>> {
    let dists = parse_arr(s)?;
    println!("{:?}", dists);
    let med = median(&dists);
    println!("Median: {}", med);
    println!("Linear cost: {}", get_cost(med, &dists));
    println!("Triangular cost: {}", brute_force(&dists));
    Ok(())
}