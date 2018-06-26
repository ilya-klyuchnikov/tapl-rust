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

#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    EvalCommand { term: Box<Term> },
}

#[test]
fn test_serde() {
    let term1 = Term::TmSucc {term: Box::new(Term::TmZero {})};
    let json1 = serde_json::to_string(&term1).unwrap();
    let term2: Term = serde_json::from_str(&json1).unwrap();
    assert_eq!(term1, term2);
}
