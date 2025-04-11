#[derive(Debug, Clone, Default, PartialEq)]
pub enum VariableState {
    #[default]
    Default,
    Processing,
    Final,
}
