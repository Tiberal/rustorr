use error_stack::ResultExt;
use error_stack::Result;
use indexmap::IndexMap;
use crate::bencode_parser::bencode_values::Value;
use crate::bencode_parser::parser::Parser;
use crate::bencode_parser::parser_error::ParserError;
use crate::torrent::torrent_error::TorrentError;

//todo remove dead code attribute
#[allow(dead_code)]
#[derive(Debug)]
pub struct Torrent {
    announce: String,
    created_by: String,
    torrent_info: TorrentInfo,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct TorrentInfo {
    name: String,
    length: i64,
    pieces: String,
    piece_length: i64,
}

impl Torrent {
    pub fn new(file_path: &str) -> Result<Self, TorrentError> {
        let byte_data = std::fs::read(file_path)
            .attach_printable("Failed to read torrent file")
            .change_context(TorrentError::TorrentFileReadError)?;

        let parser = Parser::new(byte_data);

        let data = parser.parse()
            .attach_printable("Failed to parse torrent file")
            .change_context(TorrentError::TorrentFileParseError)?;


        let torrent_data_map = &data[0].as_dictionary()
            .change_context_to_torrent_parse_error("Failed to extract announce")?;

        let announce = torrent_data_map.get("announce").unwrap().as_string()
            .change_context_to_torrent_parse_error("Failed to extract announce")?;

        let created_by = torrent_data_map.get("created by").unwrap().as_string()
            .change_context_to_torrent_parse_error("Failed to extract created by")?;

        let torrent_info_dictionary = torrent_data_map.get("info")
            .unwrap()
            .as_dictionary()
            .change_context_to_torrent_parse_error("Failed to extract info")?;


        Ok(Self {
            announce,
            created_by,
            torrent_info: TorrentInfo::new(torrent_info_dictionary)?,
        })
    }
}

impl TorrentInfo {
    fn new(info: &IndexMap<String, Value>) -> Result<Self, TorrentError> {
        let name = info.get("name").unwrap().as_string()
            .change_context_to_torrent_parse_error("Failed to extract name")?;

        let length = info.get("length").unwrap().as_number()
            .change_context_to_torrent_parse_error("Failed to extract length")?;

        let pieces = info.get("pieces").unwrap().as_string()
            .change_context_to_torrent_parse_error("Failed to extract pieces")?;

        let piece_length = info.get("piece length").unwrap().as_number()
            .change_context_to_torrent_parse_error("Failed to extract piece length")?;

        Ok(
            Self {
                name,
                length,
                pieces,
                piece_length,
            }
        )
    }
}

trait ChangeContextResultExt<T> {
    fn change_context_to_torrent_parse_error(self, printable: &str) -> Result<T, TorrentError>;
}

impl<T> ChangeContextResultExt<T> for Result<T, ParserError> {
    fn change_context_to_torrent_parse_error(self, printable: &str) -> Result<T, TorrentError> {
        self.attach_lazy(|| printable.to_owned())
            .change_context(TorrentError::TorrentFileParseError)
    }
}


