use railroad::*;
use regex_syntax::ast::{
    AssertionKind, Ast, Class, ClassPerlKind, GroupKind, LiteralKind, RepetitionKind,
    RepetitionRange, Span,
};

pub fn translate(original_pattern: &str, ast: Ast) -> Diagram<Sequence> {
    Diagram::new(Sequence::new(vec![
        Box::new(SimpleStart),
        Translator::translate(original_pattern, &ast),
        Box::new(SimpleEnd),
    ]))
}

struct Translator<'a> {
    original_pattern: &'a str,
}

impl<'a> Translator<'a> {
    fn translate<'b>(original_pattern: &'a str, ast: &'b Ast) -> Box<dyn RailroadNode> {
        Translator { original_pattern }.traverse(ast)
    }

    fn traverse<'b>(&'a self, ast: &'b Ast) -> Box<dyn RailroadNode> {
        match ast {
            Ast::Empty(_) => Box::new(Empty),
            Ast::Flags(ref f) => unimplemented!(),
            Ast::Literal(ref l) => match l.kind {
                LiteralKind::Verbatim | LiteralKind::Punctuation => {
                    Box::new(Terminal::new(l.c.to_string()))
                }
                LiteralKind::Special(ref s) => Box::new(NonTerminal::new(format!("{:?}", s))),
                _ => Box::new(Terminal::new(self.recover(&l.span).to_owned())),
            }, // TODO: Print visiable bytes directly w/o using escapes.
            Ast::Dot(_) => Box::new(NonTerminal::new(String::from("Any charaters"))),
            Ast::Assertion(ref a) => Box::new(NonTerminal::new(String::from(match a.kind {
                AssertionKind::StartLine => "start of line",
                AssertionKind::EndLine => "end of line",
                AssertionKind::StartText => "start of text",
                AssertionKind::EndText => "end of text",
                AssertionKind::WordBoundary => "word boundary",
                AssertionKind::NotWordBoundary => "non word boundary",
            }))),
            Ast::Class(ref c) => match c {
                Class::Perl(ref p) => Box::new(NonTerminal::new(format!(
                    "{}{}",
                    if p.negated { "non-" } else { "" },
                    match p.kind {
                        ClassPerlKind::Digit => "digit",
                        ClassPerlKind::Space => "whitespace",
                        ClassPerlKind::Word => "word characters",
                    }
                ))),
                _ => Box::new(NonTerminal::new(format!("{}", self.recover(c.span())))),
            },
            Ast::Repetition(ref r) => {
                let repeated = self.traverse(r.ast.as_ref());
                match r.op.kind {
                    RepetitionKind::ZeroOrOne => Box::new(Optional::new(repeated)),
                    RepetitionKind::ZeroOrMore => {
                        Box::new(Optional::new(Repeat::new(repeated, Empty)))
                    }
                    RepetitionKind::OneOrMore => Box::new(Repeat::new(repeated, Empty)),
                    RepetitionKind::Range(ref r) => match r {
                        RepetitionRange::Exactly(n) => Box::new(Repeat::new(
                            repeated,
                            Comment::new(format!("= {} times", *n - 1)),
                        )),
                        RepetitionRange::AtLeast(l) => Box::new(Repeat::new(
                            repeated,
                            Comment::new(format!("≥ {} times", *l - 1)),
                        )),
                        RepetitionRange::Bounded(l, u) => match *l {
                            0 => Box::new(Optional::new(Repeat::new(
                                repeated,
                                Comment::new(format!("≤ {} times", u - 1)),
                            ))),
                            1 => Box::new(Repeat::new(
                                repeated,
                                Comment::new(format!("≥ {} times", u - 1)),
                            )),
                            _ => Box::new(Optional::new(Repeat::new(
                                repeated,
                                Comment::new(format!("{} to {} times", l - 1, u - 1)),
                            ))),
                        },
                    },
                }
            }
            Ast::Group(ref g) => match g.kind {
                GroupKind::CaptureIndex(i) => Box::new(LabeledBox::new(
                    self.traverse(g.ast.as_ref()),
                    Comment::new(format!("Group: #{}", i)),
                )),
                GroupKind::CaptureName(ref n) => Box::new(LabeledBox::new(
                    self.traverse(g.ast.as_ref()),
                    Comment::new(format!("Group: {}", n.name)),
                )),
                GroupKind::NonCapturing(ref f) => Box::new(LabeledBox::new(
                    self.traverse(g.ast.as_ref()),
                    Comment::new(format!("Flags: {}", self.recover(&f.span))),
                )),
            },
            Ast::Alternation(ref a) => Box::new(Choice::new(
                a.asts
                    .iter()
                    .map(|ast| self.traverse(ast))
                    .collect::<Vec<Box<dyn RailroadNode>>>(),
            )),
            Ast::Concat(ref c) => Box::new(Sequence::new(
                c.asts.iter().map(|ast| self.traverse(ast)).collect(),
            )),
        }
    }

    fn recover<'b>(&'a self, span: &'b Span) -> &'a str {
        let Span { start, end } = span;
        &self.original_pattern[start.offset..end.offset]
    }
}
