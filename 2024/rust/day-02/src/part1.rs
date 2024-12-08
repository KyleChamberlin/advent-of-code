#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut safe_report_count: usize = 0;

    'report: for report in input.lines() {
        let mut levels = report
            .split_whitespace()
            .map(|l| l.parse::<isize>().unwrap());

        let first_level = levels.next().unwrap();
        let second_level = levels.next().unwrap();

        if (first_level - second_level).abs() == 0 {
            continue;
        };
        if (first_level - second_level).abs() > 3 {
            continue;
        };
        let direction: Direction = if (first_level - second_level) < 0 {
            Direction::Down
        } else {
            Direction::Up
        };
        let mut last_level = second_level;
        for level in levels {
            let diff = last_level - level;
            if diff.abs() > 3 {
                continue 'report;
            };
            if diff == 0 {
                continue 'report;
            };

            match direction {
                Direction::Up => {
                    if diff < 0 {
                        continue 'report;
                    };
                }
                Direction::Down => {
                    if diff > 0 {
                        continue 'report;
                    };
                }
            }

            last_level = level;
        }
        safe_report_count += 1
    }

    Ok(safe_report_count.to_string())
}

enum Direction {
    Up,
    Down,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
