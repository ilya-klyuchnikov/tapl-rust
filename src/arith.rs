extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize)]
pub enum Term {
    TmTrue,
    TmFalse,
    TmIf { condition: Box<Term>, term1: Box<Term>, term2: Box<Term> },
    TmZero {},
    TmSucc { term: Box<Term> },
    TmPred { term: Box<Term> },
    TmIsZero { term: Box<Term> },
}
