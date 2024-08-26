use crate::torrent::torrent::Torrent;

mod bencode_parser;
mod torrent;

fn main() {

    //todo introduce own error here
    let torrent = Torrent::new("sample.torrent").unwrap();

    println!("torrent {:?}", torrent);
}