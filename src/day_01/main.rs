use std::fs::File;
use std::io::{BufRead, BufReader};

fn file_to_lists(filepath: &str) ->std::io::Result<(Vec<i32>, Vec<i32>)> {
    let fs = File::open(filepath)?;
    let reader = BufReader::new(fs);
    let mut num_list = (Vec::new(), Vec::new());
    for line in reader.lines() {
        let s =line?;
        let mut parts = s.split_whitespace().map(|s| s.parse::<i32>());
        match (parts.next(), parts.next()) {
            (Some(Ok(a)), Some(Ok(b))) => {
                num_list.0.push(a);
                num_list.1.push(b);
            }
            _ => {}  // ignore invalid input
        }
    }
    return Ok(num_list);
}

fn part_1(filepath: &str) -> std::io::Result<i32> {
    let (mut a_list, mut b_list) = file_to_lists(filepath)?;

    a_list.sort();
    b_list.sort();

    let dist : i32 = a_list.iter().zip(b_list.iter())
        .map(|(a,b)| {
            return a - b;
        })
        .map(|x| {x.abs()})
        .sum();
    return Ok(dist);
}

fn part_2(filepath: &str) ->std::io::Result<i32> {
    /* Read lists from file */
    let (mut a_list, mut b_list) = file_to_lists(filepath)?;

    
    // sort lists
    a_list.sort();
    b_list.sort();

    let mut i = 0;
    let mut j = 0;
    let mut total = 0;
    while i < a_list.len() && j < b_list.len() {
        if a_list[i] < b_list[j] {
            i += 1;
        } else if a_list[i] > b_list[j] {
            j += 1;
        } else {
            let number_value = a_list[i];
            let mut a_count = 0;
            while i < a_list.len() && a_list[i] == number_value {
                a_count += 1;
                i += 1;
            }

            let mut b_count = 0;
            while j < b_list.len() && b_list[j] == number_value {
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
