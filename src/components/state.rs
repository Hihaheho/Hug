#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Alone,
    MatchingRandom,
    MatchingByKey,
    CreatingRoom,
    Connected,
}
