use core::num;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn part_1(filepath: &str) -> std::io::Result<i32> {
    let fs =std::fs::File::open(filepath)?;
    let mut reader = std::io::BufReader::new(fs);
    let mut num_list = [Vec::new(), Vec::new()];
    for line in reader.lines() {
        let s =line?;
        let mut parts = s.split_whitespace().map(|s| s.parse::<i32>());
        match (parts.next(), parts.next()) {
            (Some(Ok(a)), Some(Ok(b))) => {
                num_list[0].push(a);
                num_list[1].push(b);
            }
            _ => {}  // ignore invalid input
        }
    }

    num_list[0].sort();
    num_list[1].sort();

    let dist : i32 = num_list[0].iter().zip(num_list[1].iter())
        .map(|(a,b)| {
            return a - b;
        })
        .map(|x| {x.abs()})
        .sum();
    return Ok(dist);
}

fn part_2(filepath: &str) ->std::io::Result<i32> {
    /* Read lists from file */
    let fs =std::fs::File::open(filepath)?;
    let mut reader = std::io::BufReader::new(fs);
    let mut num_list = [Vec::new(), Vec::new()];
    for line in reader.lines() {
        let s =line?;
        let mut parts = s.split_whitespace().map(|s| s.parse::<i32>());
        match (parts.next(), parts.next()) {
            (Some(Ok(a)), Some(Ok(b))) => {
                num_list[0].push(a);
                num_list[1].push(b);
            }
            _ => {}  // ignore invalid input
        }
    }
    
    // sort lists
    num_list[0].sort();
    num_list[1].sort();

    let mut i = 0;
    let mut j = 0;
    let mut total = 0;
    while i < num_list[0].len() && j < num_list[1].len() {
        if num_list[0][i] < num_list[1][j] {
            i += 1;
        } else if num_list[0][i] > num_list[1][j] {
            j += 1;
        } else {
            let number_value = num_list[0][i];
            let mut a_count = 0;
            while i < num_list[0].len() && num_list[0][i] == number_value {
                a_count += 1;
                i += 1;
            }

            let mut b_count = 0;
            while j < num_list[1].len() && num_list[1][j] == number_value {
                b_count += 1;
                j += 1;
            }

            let score = number_value*a_count*b_count;
            total += score;
        }
    }

    return Ok(total);

}

fn main() -> std::io::Result<()> {
    let ans_p1 = part_1("src/day_01/input.txt")?;
    println!("part 1: {:?}", ans_p1);
    
    let ans_p2 = part_2("src/day_01/input.txt")?;
    println!("part 2: {:?}", ans_p2);

    return Ok(())
}  

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() -> std::io::Result<()> {
        let result = part_1("src/day_01/example.txt")?;
        assert_eq!(result, 11);
        return Ok(())
    }

    #[test]
    fn test_part2() -> std::io::Result<()> {
        let result = part_2("src/day_01/example.txt")?;
        assert_eq!(result, 31);
        return Ok(())
    }
}
