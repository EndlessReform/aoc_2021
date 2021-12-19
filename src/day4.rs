use crate::Error;
use std::collections::HashMap;

struct Board {
    ns: HashMap<i32, (i32, i32, bool)>,
    ncalled_r: [i32; 5],
    ncalled_c: [i32; 5],
    won: bool,
}

impl Board {
    fn call(&mut self, n: i32) -> Option<i32> {
        if let Some(m) = self.ns.get_mut(&n) {
            // We called a number!
            let r = m.0 as usize;
            let c = m.1 as usize;
            self.ncalled_r[r] += 1;
            self.ncalled_c[c] += 1;

            // Bingo!
            if self.ncalled_c[c] == 5 || self.ncalled_r[r] == 5 {
                let mut sum = 0;
                for (k, v) in self.ns.iter() {
                    if !v.2 {
                        sum += k;
                    }
                }
                self.won = true;
                Some(sum)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn new() -> Board {
        Board {
            ns: HashMap::new(),
            ncalled_r: [0; 5],
            ncalled_c: [0; 5],
            won: false,
        }
    }
}

pub fn day4(s: &str) -> Result<(), Error> {
    let mut lines = s.split_terminator('\n');

    // Get called numbers
    let called: Vec<i32>;
    match lines.next() {
        Some(s) => {
            called = s
                .split_terminator(',')
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
        }
        None => return Err(Error::Parse),
    }

    // Eat up boards
    let mut boards: Vec<Board> = Vec::new();
    let mut nrows_filled = 0;

    for line in lines {
        // Newline: new board
        if line.is_empty() {
            nrows_filled = 0;
            boards.push(Board::new());
        } else {
            // This will never panic since there should be initial newline
            let board = boards.last_mut().unwrap();

            nrows_filled += 1;
            let raw_nums = line.split_whitespace();

            for (i, n) in raw_nums.enumerate() {
                let x = n.parse::<i32>()?;
                board.ns.insert(x, (nrows_filled - 1, i as i32, false));
            }
        }
    }

    // Score: All unmarked numbers * last called number
    let mut last: i32 = 0;

    for n in called {
        for board in &mut boards {
            if !board.won {
                if let Some(sum) = board.call(n) {
                    println!("{}", sum);
                    last = n * sum;
                }
            }
        }
    }
    println!("Score of last win: {}", last);
    Ok(())
}
