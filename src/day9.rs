use std::collections::HashSet;
use std::error::Error;

type Point = (usize, usize);
type Matrix = Vec<Vec<u8>>;

fn parse_matrix(s: &str) -> Result<Matrix, Box<dyn Error>> {
    let mut res: Matrix = Vec::new();

    for line in s.lines() {
        let mut tmp: Vec<u8> = Vec::new();
        for c in line.chars() {
            tmp.push(c.to_string().parse::<u8>()?)
        }
        res.push(tmp);
    }
    Ok(res)
}

/// ncols, nrows
fn get_dimensions(matrix: &Matrix) -> (usize, usize) {
    (matrix.len(), matrix[0].len())
}

fn get_neighbors(matrix: &Matrix, (r, c): Point) -> std::vec::IntoIter<Point> {
    let (nrows, ncols) = get_dimensions(matrix);
    let mut res: Vec<Point> = Vec::new();
    if r > 0 {
        res.push((r - 1, c))
    };
    if c > 0 {
        res.push((r, c - 1))
    };
    if r < (nrows - 1) {
        res.push((r + 1, c))
    };
    if c < (ncols - 1) {
        res.push((r, c + 1))
    };
    res.into_iter()
}

fn get_lowpoints(matrix: &Matrix) -> std::vec::IntoIter<Point> {
    let mut res = Vec::new();
    let (nrows, ncols) = get_dimensions(matrix);

    for r in 0..nrows {
        for c in 0..ncols {
            if get_neighbors(matrix, (r, c))
                .all(|(nr, nc)| matrix[nr][nc] > matrix[r][c])
            {
                res.push((r, c));
            }
        }
    }
    res.into_iter()
}

/// Mostly derived from redditonlyforu's solution
fn basin_size(matrix: &Matrix, start: &Point) -> i32 {
    let mut stack: Vec<Point> = vec![*start];
    let mut visited: HashSet<Point> = HashSet::new();

    while let Some((r, c)) = stack.pop() {
        if !visited.insert((r,c)) {
            continue;
        }
        get_neighbors(matrix, (r, c))
            .filter(|&(nr,nc)| matrix[nr][nc] != 9 && matrix[nr][nc] > matrix[r][c])
            .for_each(|node| {
                stack.push(node)
            });
    }
    visited.len() as i32
}

fn part1(matrix: &Matrix) -> Result<i32, Box<dyn Error>> {
    Ok(get_lowpoints(matrix).fold(0, |acc, (r, c)| acc + 1 + matrix[r][c] as i32))
}

fn part2(matrix: &Matrix) -> Result<i32, Box<dyn Error>> {
    // Identify basins
    let mut basin_sizes: Vec<i32> = get_lowpoints(matrix)
        .map(|point| basin_size(matrix, &point))
        .collect();

    basin_sizes.sort_unstable();
    basin_sizes.reverse();
    basin_sizes.truncate(3);
    Ok(basin_sizes.iter().product())
}

/// Assume sane input for all functions
pub fn day9(s: &str) -> Result<(), Box<dyn Error>> {
    let matrix = parse_matrix(s)?;
    println!("Sum: {}", part1(&matrix)?);
    println!("Product: {}", part2(&matrix)?);
    Ok(())
}
