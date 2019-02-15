extern crate railroad;
extern crate regex_syntax;

mod translator;

use self::translator::translate;

use railroad::{svg, Diagram, Sequence, DEFAULT_CSS};
use regex_syntax::ast::parse::Parser;
pub use regex_syntax::Result;

pub fn generate_diagram<T: AsRef<str>>(regex: T) -> Result<Diagram<Sequence>> {
    let regex = regex.as_ref();
    let mut dia = translate(regex, Parser::new().parse(regex)?);

    dia.add_element(
        svg::Element::new("style")
            .set("type", "text/css")
            .text(DEFAULT_CSS),
    );

    Ok(dia)
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
