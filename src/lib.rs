#[macro_use]
extern crate railroad;
extern crate regex_syntax;

mod converter;

use self::converter::convert;

use railroad::Diagram;
use regex_syntax::{Parser, Result};

fn generate_diagram<T: AsRef<str>>(regex: T) -> Result<Diagram> {
    Ok(convert(Parser::new().parse(regex.as_ref())?))
}


// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }