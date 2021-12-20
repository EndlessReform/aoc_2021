use std::collections::HashMap;
use std::cmp::{min, max};
use std::error::Error;

type Point = (i32,i32);


fn parse_to_point(s: &str) -> Result<Point, Box<dyn Error>> {
    let x: Vec<&str> = s.split(',').collect();
    if x.len() != 2 {
        Err("Wrong number number".into())
    } else {
        Ok((x[0].parse::<i32>()?, x[1].parse::<i32>()?))
    }
}

fn parse_to_points(line: &str) -> Result<(Point, Point), Box<dyn Error>> {
    // Get points
    let ps:Vec<&str> = line.split_terminator(" -> ").collect();
    if ps.len() != 2 { return Err("Wrong number of points".into()) };
    let p1 = parse_to_point(ps[0])?;
    let p2 = parse_to_point(ps[1])?;
    Ok((p1,p2))
}

// Assume vertical or horizontal
fn enumerate_line(p1: &Point, p2: &Point) -> Vec<Point> {
    let mut res = Vec::new();

    if p1.0 == p2.0 {
        // Vertical
        for y in min(p1.1,p2.1)..max(p1.1,p2.1)+1 {
            res.push((p1.0, y));
        }
    } else if p1.1 == p2.1 {
        // Horizontal
        for x in min(p1.0,p2.0)..max(p1.0,p2.0)+1 {
            res.push((x, p1.1));
        }
    } else {
        // Diagonal
        let y_incr = p1.1 < p2.1;
        let x_incr = p1.0 < p2.0;
        let line_is_up: bool = x_incr ^ y_incr == false;

        if line_is_up {
            if x_incr {
                //println!("x inc, y inc");
                res = (p1.0..p2.0+1).zip(p1.1..p2.1+1).collect();
            } else {
                //println!("x dec, y dec");
                res = (p2.0..p1.0+1).zip(p2.1..p1.1+1).collect();
            }
        } else {
            if x_incr {
                //println!("x inc, y dec");
                res = (p1.0..p2.0+1).zip((p2.1..p1.1+1).rev()).collect();
            } else {
                //println!("x dec, y inc");
                res = (p2.0..p1.0+1).zip((p1.1..p2.1+1).rev()).collect();
            }
        }
    }
    res
}


pub fn day5(s: &str) -> Result<(), Box<dyn Error>> {
    let mut points = HashMap::<Point,i32>::new();

    // Add points
    for line in s.lines() {
        let (p1, p2) = parse_to_points(line)?;
        // Increase visit count for all points in line
        let el = enumerate_line(&p1, &p2);
        for p in el {
            *points.entry(p).or_insert(0) += 1;
        }
    }

    // Count how many have count >2
    let nvisited = points.values()
        .fold(0, |sum, &x| if x >= 2 { sum + 1 } else {sum});

    println!("Above 2: {}", nvisited);
    Ok(())
}
