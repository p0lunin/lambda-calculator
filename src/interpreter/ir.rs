use crate::syntax::ast;
pub use crate::syntax::ast::Ident;
use std::ops::Deref;
use std::fmt::{Display, Formatter};

pub fn term_to_ir(term: &ast::Term) -> Box<Term> {
    Box::new(match term {
        ast::Term::Lambda(l) => Term::Lambda(lambda_to_ir(l)),
        ast::Term::Unit(u) => Term::Unit(unit_to_ir(u)),
        ast::Term::Application(a) => Term::Application(appl_to_ir(a)),
        ast::Term::Brackets(b) => return term_to_ir(b.data.deref())
    })
}

fn appl_to_ir(appl: &ast::Application) -> Application {
    let ast::Application(l, r) = appl;
    Application(term_to_ir(l), term_to_ir(r))
}

fn lambda_to_ir(lambda: &ast::Lambda) -> Lambda {
    let ast::Lambda(var, body) = lambda;
    Lambda((*var).clone().inner(), term_to_ir(body.deref()))
}

fn unit_to_ir(unit: &ast::Unit) -> Unit {
    match unit {
        ast::Unit::Ident(i) => Unit::Ident(i.clone().inner()),
        ast::Unit::Lit(l) => Unit::Lit(lit_to_ir(l.deref())),
    }
}

fn lit_to_ir(lit: &ast::Literal) -> Literal {
    match lit {
        ast::Literal::Int(i) => Literal::Int(i.clone()),
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Term {
    Lambda(Lambda),
    Unit(Unit),
    Application(Application),
}

impl Display for Term {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Term::Lambda(t) => t.fmt(f),
            Term::Unit(t) => t.fmt(f),
            Term::Application(t) => t.fmt(f),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Lambda(pub Ident, pub Box<Term>);

impl Display for Lambda {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "(\\{}.{})", self.0.0, self.1)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Application(pub Box<Term>, pub Box<Term>);

impl Display for Application {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{} {}", self.0, self.1)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Unit {
    Ident(Ident),
    Lit(Literal),
}

impl Display for Unit {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Unit::Ident(i) => f.write_str(i.0.as_str()),
            Unit::Lit(l) => l.fmt(f)
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Int(i64),
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Literal::Int(i) => i.fmt(f),
        }
    }
}
