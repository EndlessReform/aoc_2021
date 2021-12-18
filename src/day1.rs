pub fn day1(s: String) -> std::io::Result<()> {
    let mut prev:(i32,i32,i32) = (0,0,0);

    let mut num_increases:i32 = -3;

    for n in s.split_whitespace() {
        let curr = n.parse::<i32>();
        match curr {
            Ok(n) => {
                if (prev.1 + prev.2 + n) > (prev.0 + prev.1 + prev.2) {
                    num_increases += 1;
                };
                prev = (prev.1, prev.2, n);
            },
            _ => continue,
        }
    }

    println!("Number of increases: {}", num_increases);
    Ok(())
}