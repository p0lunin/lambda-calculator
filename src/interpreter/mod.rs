mod ir;

use std::ops::Deref;
use crate::interpreter::ir::{Term, Application, Ident, Unit, Lambda, term_to_ir};
use crate::syntax;

#[derive(Debug)]
pub struct Interpreter {

}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {}
    }
}

impl Interpreter {
    /*pub fn interpret(&self, term: Term) -> Result<Vec<Term>, String> {

    }*/
    pub fn run_code(&self, code: &str) -> Result<String, String> {
        let ast = syntax::parse_term(code).map_err(|e| e.to_string())?;
        let ir = term_to_ir(ast.as_ref().deref());
        let res = self.one_step_beta_reduction(*ir);
        Ok(res.to_string())
    }

    pub fn one_step_beta_reduction(&self, term: Term) -> Term {
        match term {
            Term::Lambda(l) => Term::Lambda(l),
            Term::Unit(u) => Term::Unit(u),
            Term::Application(a) => {
                let Application(l, r) = a;
                Self::application(*l, *r)
            }
        }
    }
    fn application(left: Term, right: Term) -> Term {
        match left {
            Term::Lambda(l) => {
                let Lambda(arg, mut term) = l;
                Self::apply_arg(term.as_mut(), &arg, &right);
                *term
            }
            Term::Unit(u) => Term::Unit(u),
            Term::Application(a) => Term::Application(a),
        }
    }
    fn apply_arg(to: &mut Term, arg: &Ident, term: &Term) {

        match to {
            Term::Lambda(l) => {
                if l.0 != *arg {
                    Self::apply_arg(l.1.as_mut(), arg, term)
                }
            }
            Term::Unit(u) => {
                match u {
                    Unit::Ident(id) if id == arg => {
                        *to = term.clone();
                    }
                    _ => {}
                }
            }
            Term::Application(Application(l, r)) => {
                Self::apply_arg(l.as_mut(), arg, term);
                Self::apply_arg(r.as_mut(), arg, term);
            }
        }
    }
}
