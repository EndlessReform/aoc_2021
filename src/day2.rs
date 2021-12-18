use crate::Error;

pub fn day2(s: String) -> Result<(), Error> {
    // x/y coords
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;

    let lines = s.split_terminator('\n');

    for line in lines {
        let words: Vec<&str> = line.split_whitespace().collect();
        if words.len() != 2 {
            return Err(Error::Parse);
        }

        let dir = words[0];
        let amt: i32 = words[1].parse::<i32>()?;

        match dir {
            "forward" => {
                x += amt;
                y += aim * amt
            }
            "down" => aim += amt,
            "up" => aim -= amt,
            _ => {}
        }
    }

    println!("x: {}, y: {}", x, y);
    println!("Product: {}", x * y);

    Ok(())
}
