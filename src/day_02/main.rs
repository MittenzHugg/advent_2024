use std::fs::{read, File};
use std::io::{BufRead, BufReader};

fn report_is_safe(report: &[i32]) -> bool {
    let diff = report.windows(2)
        .map(|x| x[0] - x[1]);
    let increasing = diff.clone().all(|x| x <=3 && x > 0);
    let decreasing = diff.clone().all(|x| -3 <= x && x < 0);
    return increasing | decreasing;
}

fn report_is_safe_dampened(report: &[i32]) -> bool {
    for i in 0.. report.len() {
        let dampened_report= report[..i].iter().chain(report[i + 1 ..].iter()).cloned().collect::<Vec<i32>>();
        if report_is_safe(&dampened_report) {
            return true;
        }
    }
    return false;
}

fn part_1(filepath: &str) ->std::io::Result<i32> {
    let fs = File::open(filepath)?;
    let reader = BufReader::new(fs);
    let reports = reader.lines().flatten()
        .map(|line| line.split_whitespace().flat_map(|s| s.parse::<i32>())
        .collect::<Vec<i32>>());

    let safe_number = reports.filter(|r| report_is_safe(&r))
        .count();

    return Ok(safe_number as i32);
}

fn part_2(filepath: &str) ->std::io::Result<i32> {
    let fs = File::open(filepath)?;
    let reader = BufReader::new(fs);
    let reports = reader.lines().flatten()
        .map(|line| line.split_whitespace().flat_map(|s| s.parse::<i32>())
        .collect::<Vec<i32>>());

    let safe_number = reports.filter(|r| report_is_safe_dampened(&r))
        .count();

    return Ok(safe_number as i32);
}


fn main() -> std::io::Result<()> {
    let ans_p1 = part_1("src/day_02/input.txt")?;
    println!("part 1: {:?}", ans_p1);
    
    let ans_p2 = part_2("src/day_02/input.txt")?;
    println!("part 2: {:?}", ans_p2);

    return Ok(())
}  

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() -> std::io::Result<()> {
        let result = part_1("src/day_02/example.txt")?;
        assert_eq!(result, 2);
        return Ok(())
    }

    #[test]
    fn test_part2() -> std::io::Result<()> {
        let result = part_2("src/day_02/example.txt")?;
        assert_eq!(result, 4);
        return Ok(())
    }
}
