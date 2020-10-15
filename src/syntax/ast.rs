use crate::common::{Span, Sp};

#[derive(Debug, PartialEq)]
pub enum Term {
    Lambda(Sp<Lambda>),
    Unit(Sp<Unit>),
    Application(Sp<Application>),
    Brackets(Sp<Brackets<Box<Sp<Term>>>>),
}

#[derive(Debug, PartialEq)]
pub struct Lambda(pub Sp<Ident>, pub Box<Sp<Term>>);

#[derive(Debug, PartialEq)]
pub struct Application(pub Box<Sp<Term>>, pub Box<Sp<Term>>);

#[derive(Debug, PartialEq, Clone)]
pub struct Ident(pub String);

#[derive(Debug, PartialEq)]
pub enum Unit {
    Ident(Sp<Ident>),
    Lit(Sp<Literal>),
}

#[derive(Debug, PartialEq)]
pub struct Brackets<T> {
    pub start_bracket: Span,
    pub data: Sp<T>,
    pub end_bracket: Span,
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Int(i64),
}
