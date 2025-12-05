use std::io::Result;

use crate::Input;

pub fn main(_input: &Input) -> Result<usize> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    const EXAMPLE_DATA: &str = include_str!("../../input/day_{{day}}_example");

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input(EXAMPLE_DATA))?, 0);
        Ok(())
    }
}
