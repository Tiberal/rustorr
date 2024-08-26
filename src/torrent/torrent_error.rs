use std::fmt::{Display, Formatter};
use error_stack::Context;

#[derive(Debug)]
pub(crate) enum TorrentError {
    TorrentFileReadError,
    TorrentFileParseError,
}

impl Display for TorrentError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            TorrentError::TorrentFileReadError => "Torrent file read error",
            TorrentError::TorrentFileParseError => "Torrent file parse error",
        };
        write!(f, "{msg}")
    }
}

impl Context for TorrentError {}