use super::InfixAstTreeApi;

pub mod implementation;

pub trait InfixAstTreeApiLen: InfixAstTreeApi {
    fn len(&self) -> usize;
}
