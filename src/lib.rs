extern crate railroad;
extern crate regex_syntax;

mod translator;

use self::translator::translate;

use railroad::{Diagram, Sequence};
use regex_syntax::ast::parse::Parser;
pub use regex_syntax::Result;

pub fn generate_diagram<T: AsRef<str>>(regex: T) -> Result<Diagram<Sequence>> {
    let regex = regex.as_ref();
    Ok(translate(regex, Parser::new().parse(regex)?))
}


// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }