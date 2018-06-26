extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Term {
    TmTrue { },
    TmFalse { },
    TmIf { condition: Box<Term>, term1: Box<Term>, term2: Box<Term> },
    TmZero { },
    TmSucc { term: Box<Term> },
    TmPred { term: Box<Term> },
    TmIsZero { term: Box<Term> },
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Command {
    EvalCommand { term: Box<Term> },
}

pub fn is_numeric_val(t: &Term) -> bool {
    match t {
        Term::TmZero {} =>
            true,
        Term::TmSucc { term } =>
            is_numeric_val(term.as_ref()),
        _ => false,
    }
}

pub mod big_step_evaluator {
    use arith::*;
    use arith::Term::*;

    // this is a version when we desctruct and starts owning the passed value
    pub fn eval(t: Term) -> Term {

        match t {
            TmTrue {} => t,
            TmFalse {} => t,
            TmZero {} => t,
            TmIf { condition, term1, term2 } => match eval(*condition) {
                TmTrue {} => eval(*term1),
                TmFalse {} => eval(*term2),
                _ => panic!("No Rule Applies"),
            }
            TmSucc {term} => {
                let t2 = eval(*term);
                if is_numeric_val(&t2) {
                    TmSucc {term : Box::new(t2)}
                } else {
                    panic!("No Rule Applies")
                }
            }
            TmPred {term} => {
                match eval(*term) {
                    TmZero {} => TmZero {},
                    TmSucc {term} => if is_numeric_val(&term) {
                        *term
                    } else {
                        panic!("No Rule Applies")
                    }
                    _ => panic!("No Rule Applies"),
                }
            }
            TmIsZero {term} => {
                match eval(*term) {
                    TmZero {} => TmTrue {},
                    TmSucc {term} => if is_numeric_val(&term) {
                        TmFalse {}
                    } else {
                        panic!("No Rule Applies")
                    }
                    _ => panic!("No Rule Applies")
                }
            }
        }

    }
}

#[test]
fn test_serde() {
    let term1 = Term::TmSucc {term: Box::new(Term::TmZero {})};
    let json1: String = serde_json::to_string(&term1).unwrap();
    let term2: Term = serde_json::from_str(&json1).unwrap();
    assert_eq!(term1, term2);
}
