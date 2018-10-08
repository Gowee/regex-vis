use railroad::*;
use regex_syntax::hir::*;

/// regex_syntax::Hir to railroad::Diagram converter  
pub fn convert(hir: Hir) -> Diagram<Sequence> {
    Diagram::new(Sequence::new(vec![
        Box::new(Start),
        cnvt(hir),
        Box::new(End),
    ]))
}

fn cnvt(hir: Hir) -> Box<RailroadNode> {
    match hir.into_kind() {
        HirKind::Empty => Box::new(Empty),
        HirKind::Literal(literal) => Box::new(match literal {
            Literal::Unicode(c) => Terminal::new(c.to_string()),
            // TODO: Print visiable bytes directly w/o using escapes.
            Literal::Byte(b) => Terminal::new(format!("\\x{:x}", b)),
        }),
        HirKind::Class(c) => {
            unimplemented!();
        }
        _ => {
            unimplemented!();
        }
    }
}
