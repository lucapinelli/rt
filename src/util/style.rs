use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub enum Style {
    NAME,
    RELATIVE,
    ABSOLUTE,
}

impl FromStr for Style {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "name" => return Ok(Style::NAME),
            "relative" => return Ok(Style::RELATIVE),
            "absolute" => return Ok(Style::ABSOLUTE),
            _ => Err(String::from(
                "Expected one of the following values: name, relative or absolute",
            )),
        }
    }
}
