use std::collections::HashMap;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut left = vec![];
    let mut right = vec![];

    for line in input.lines() {
        let mut items = line.split_whitespace();

        left.push(items.next().unwrap().parse::<isize>().unwrap());

        right.push(items.next().unwrap().parse::<isize>().unwrap());
    }

    left.sort();
    right.sort();

    let right_counts: HashMap<isize, isize> = right.iter().fold(HashMap::new(), |mut counts, n| {
        *counts.entry(n.to_owned()).or_insert(0) += 1;
        counts
    });

    let result: isize = left
        .iter()
        .map(|l| (l * right_counts.get(l).unwrap_or(&0)))
        .sum();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!("31", process(input)?);
        Ok(())
    }
}
