use crate::{
    common::{Span, Sp},
    syntax::ast::*,
};

peg::parser! { grammar lang() for str {

    // TERM

    pub rule term() -> Box<Sp<Term>> =
        t:sp(<sp(<lambda()>)>) { Box::new(t.map(Term::Lambda)) } /
        t:sp(<sp(<application()>)>) { Box::new(t.map(Term::Application)) } /
        t:sp(<sp(<brackets(<term()>)>)>) { Box::new(t.map(Term::Brackets)) } /
        t:sp(<unit()>) { Box::new(t.map(Term::Unit)) }

    // to avoid application
    rule term_without_application() -> Box<Sp<Term>> =
        t:sp(<sp(<lambda()>)>) { Box::new(t.map(Term::Lambda)) } /
        t:sp(<sp(<brackets(<term()>)>)>) { Box::new(t.map(Term::Brackets)) } /
        t:sp(<unit()>) { Box::new(t.map(Term::Unit)) }

    rule application() -> Application =
        l:term_without_application() _ r:term()
    {
        Application(l, r)
    }

    rule lambda() -> Lambda =
        "\\" _ id:ident() _ "." _ t:term()
    {
        Lambda(id, t)
    }

    // UNIT

    rule unit() -> Sp<Unit> =
        l:sp(<lit()>) { l.map(Unit::Lit) } /
        id:sp(<ident()>) { id.map(Unit::Ident) }

    rule lit() -> Sp<Literal> =
        i:int()
    {
        i.map(Literal::Int)
    }

    rule ident() -> Sp<Ident> =
        ident:sp(<$(['a'..='z'|'A'..='Z'|'_'] ['a'..='z'|'A'..='Z'|'0'..='9'|'_'|'\'']*)>)
    {
        ident.map(|s| Ident(s.to_string()))
    }

    // COMMON

    rule brackets<T>(r: rule<T>) -> Brackets<T> =
        start:sp(<"(">) _ t:sp(<r()>) _ end:sp(<")">)
    {
        Brackets {
            start_bracket: start.span,
            data: t,
            end_bracket: end.span,
        }
    }
    rule sp<T>(r: rule<T>) -> Sp<T>
        = s:position!() t:r() e:position!()
    {
        Sp::new(t, Span::new(s, e))
    }
    rule int() -> Sp<i64>
        = i:sp(<$(['0'..='9']+)>)
    {
        i.map(|s| s.parse().unwrap())
    }
    rule _() = [' '|'\n'|'\t'|'\r']*
}}

pub fn parse_term(s: &str) -> Result<Box<Sp<Term>>, peg::error::ParseError<peg::str::LineCol>> {
    lang::term(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number() {
        assert!(
            parse_term(r"1234567890").is_ok()
        )
    }

    #[test]
    fn parse() {
        assert!(
            parse_term(r"\x.x(\y.y z 5)").is_ok()
        )
    }
}
