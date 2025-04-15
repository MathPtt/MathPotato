#[derive(Debug, Clone, Eq, Default, PartialEq)]
pub enum VariableState {
    #[default]
    Default,
    Processing,
    Final,
}
