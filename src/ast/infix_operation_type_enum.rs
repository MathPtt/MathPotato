#[derive(Clone, PartialEq, Eq, Debug)]
pub enum InfixOperationTypeEnum {
    Addition,
    Multiplication,
}

impl InfixOperationTypeEnum {
    pub fn precedence(&self) -> i32 {
        match self {
            Self::Multiplication => 9,
            Self::Addition => 10,
        }
    }
    pub fn has_higher_precedence(&self, other: &Self) -> bool {
        self.precedence() > other.precedence()
    }
}
