#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    todo!("day 01 - part 2");
}

fn parse(_input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        todo!("haven't built test yet");
    }

    #[test]
    fn parse_simple() -> miette::Result<()> {
        let input = "mul(1,2)";
        assert_eq!("".to_string(), parse(input));
        Ok(())
    }
}
