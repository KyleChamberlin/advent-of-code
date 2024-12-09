use nom::{
    bytes::complete::tag,
    character::complete::{self, anychar},
    multi::{many1, many_till},
    sequence::{delimited, separated_pair},
    IResult, Parser, ToUsize,
};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    Ok(parse(input).iter().map(|exp| exp.evaluate()).sum::<usize>().to_string())
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Operation {
    Multiply,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
struct Expression {
    operation: Operation,
    left: usize,
    right: usize,
}

impl Expression {
    fn evaluate(self) -> usize {
        match self.operation {
            Operation::Multiply => self.left * self.right,
        }
    }
}

fn parse_expression(input: &str) -> IResult<&str, Expression> {
    let (input, _) = tag("mul")(input)?;
    let (input, pair) = delimited(
        tag("("),
        separated_pair(complete::u32, tag(","), complete::u32),
        tag(")"),
    )(input)?;

    Ok((
        input,
        Expression {
            operation: Operation::Multiply,
            left: pair.0.to_usize(),
            right: pair.1.to_usize(),
        },
    ))
}

fn parse(input: &str) -> Vec<Expression> {
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
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!("161", process(input)?);
        Ok(())
    }

    #[test]
    fn parse_simple_expression() -> miette::Result<()> {
        let input = "mul(1,2)";
        assert_eq!(
            parse_expression(input),
            Ok((
                "",
                Expression {
                    operation: Operation::Multiply,
                    left: 1,
                    right: 2,
                }
            ))
        );
        Ok(())
    }
    #[test]
    fn parse_simple() -> miette::Result<()> {
        let input = "mul(1,2)";
        assert_eq!(
            parse(input),
            vec![Expression {
                operation: Operation::Multiply,
                left: 1,
                right: 2,
            }]
        );
        Ok(())
    }

#[test]
fn expression() -> miette::Result<()> {
        assert_eq!(Expression { operation: Operation::Multiply, left: 2, right: 3}.evaluate(), 6);
        Ok(())
    }
}
