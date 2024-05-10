/// Enumerates valid game play statuses.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub(crate) enum LocalPlayStatus {
    EndedInStalemate,
    EndedInWin,
    InProgress,
    #[default]
    NotStarted,
}
