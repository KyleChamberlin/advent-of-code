
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, anychar},
    combinator::value,
    multi::{many1, many_till},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut on = true;
    let mut sum = 0;
    for exp in parse(input) {
        match exp {
            Operation::Multiply(a, b) => {
                if on {
                    sum += a * b;
                }
            }
            Operation::Dont => on = false,
            Operation::Do => on = true,
        }
    }

    Ok(sum.to_string())

}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Operation {
    Multiply(u32, u32),
    Dont,
    Do,
}

fn mul(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("mul")(input)?;
    let (input, pair) = delimited(
        tag("("),
        separated_pair(complete::u32, tag(","), complete::u32),
        tag(")"),
    )(input)?;

    Ok((input, Operation::Multiply(pair.0, pair.1)))
}

fn parse_expression(input: &str) -> IResult<&str, Operation> {
    alt((
        value(Operation::Do, tag("do()")),
        value(Operation::Dont, tag("don't()")),
        mul,
    ))(input)
}

fn parse(input: &str) -> Vec<Operation> {
    if let Ok((_input, expressions)) =
        many1(many_till(anychar, parse_expression).map(|(_discard, ins)| ins))(input)
    {
        return expressions;
    }
    panic!("nope");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!("48", process(input)?);
        Ok(())
    }

    #[test]
    fn parse_simple_expression() -> miette::Result<()> {
        let input = "mul(1,2)";
        assert_eq!(
            parse_expression(input),
            Ok(("", Operation::Multiply(1, 2)))
        );
        Ok(())
    }
    #[test]
    fn parse_simple() -> miette::Result<()> {
        let input = "mul(1,2)";
        assert_eq!(
            parse(input),
            vec![Operation::Multiply(1,2)]
        );
        Ok(())
    }
}
