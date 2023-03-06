#![feature(option_result_contains)]
#![feature(iter_next_chunk)]

use select::document::Document;
use crate::youtube::Video;

pub mod utils;
pub mod youtube;

// temp hacks
const FILENAME : &str = "../watch-history.html";
fn main() {

    let document = Document::from(include_str!("../watch-history.html"));
    let my_videos = youtube::parse_youtube_takeout(&document);

    println!("{}", document.nodes.len());

    for i in 0..10 {
        println!("{:?}", my_videos[i]);
    }

//    println!("{my_videos}");

}
