use core::panic;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut safe_report_count: usize = 0;

    for report in input.lines() {
        let levels: Vec<isize> = report
            .split_whitespace()
            .map(|l| l.parse::<isize>().unwrap())
            .collect();

        if is_safe_with_exception(levels) {
            safe_report_count += 1;
        } else {
            eprintln!("{}", report)
        }
    }

    Ok(safe_report_count.to_string())
}

fn is_safe_with_exception(levels: Vec<isize>) -> bool {
    match is_safe(&levels) {
        Status::Safe => true,
        Status::BadLevel(i) => {
            for idx in 0..levels.len() {
                if is_safe(&levels.without(idx)) == Status::Safe {
                    return true;
                }
            }
            return false;
        }
    }
}

trait Without {
    fn without(&self, i: usize) -> Vec<isize>;
}
impl Without for Vec<isize> {
    fn without(&self, i: usize) -> Vec<isize> {
        if i == 0 {
            eprintln!("removing 0: {:?}", self);
            return self[1..].to_vec();
        }
        if i == (self.len() - 1) {
            eprintln!("removing {}: {:?}", i, self);
            return self[..i].to_vec();
        }
        if i >= self.len() {
            panic!("you done fucked up")
        }
        let mut vec =  self.clone();
        vec.remove(i);

        vec
    }
}

fn is_safe(levels: &[isize]) -> Status {
    let mut level_iter = levels.iter().enumerate();
    let (_, mut last_level) = level_iter.next().unwrap();
    let mut direction: Direction = Direction::Undefined;
    for (i, level) in level_iter {
        let diff = last_level - level;
        if diff.abs() > 3 || diff == 0 {
            return Status::BadLevel(i);
        };
        match direction {
            Direction::Down => {
                if diff < 0 {
                    return Status::BadLevel(i);
                };
            }
            Direction::Up => {
                if diff > 0 {
                    return Status::BadLevel(i);
                };
            }
            Direction::Undefined => {
                if diff > 0 {
                    direction = Direction::Down
                } else {
                    direction = Direction::Up
                }
            }
        };
        last_level = level;
    }
    Status::Safe
}

#[derive(Debug, Eq, PartialEq)]
enum Status {
    Safe,
    BadLevel(usize),
}

#[derive(Debug)]
enum Direction {
    Undefined,
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
        assert_eq!("4", process(input)?);
        Ok(())
    }

    #[test]
    fn change_in_direction() -> miette::Result<()> {
        let input = "61 60 62 64 65 66 69";

        assert_eq!("1", process(input)?);
        Ok(())
    }

    #[test]
    fn change_in_direction_remove_first() -> miette::Result<()> {
        let input = "3 1 2 4 5";

        assert_eq!("1", process(input)?);
        Ok(())
    }

    #[test]
    fn bug() -> miette::Result<()> {
        let input = "3 1 2 4 5";

        assert_eq!("1", process(input)?);
        Ok(())
    }
}
