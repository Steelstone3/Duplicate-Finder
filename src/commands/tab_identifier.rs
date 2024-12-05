#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub enum TabIdentifier {
    #[default]
    Editor,
    DuplicateFinder,
}