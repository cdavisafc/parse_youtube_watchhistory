use chrono::NaiveDateTime;
use select::document::{self, Document};
use select::node::{Data, Node, Raw};
use select::predicate::{And, Attr, Class, Descendant, Name, Predicate};
use std::borrow::Borrow;
use std::iter::OnceWith;
use std::mem;
use std::sync::mpsc::channel;

//use crate::utils;
// utils;

#[derive(Debug)]
pub struct Video<'a> {
    pub video_url: &'a str,
    pub video_title: &'a str,
    pub channel_url: &'a str,
    pub channel_title: &'a str,
    pub date_watched: NaiveDateTime,
}

pub fn parse_youtube_takeout<'a>() -> (Document, Vec<Video<'a>>) {
    let mut my_videos = Vec::new();
    //    let s = String::from("foobar");

    let document = Document::from(include_str!("../watch-history.html"));
    println!("loaded doc - num nodes {}", document.nodes.len());

    // The following finds divs with class outer-cell (this is an entry in the list that represents a video) AND
    // does not contain the string "Google Ad" in the descendants - that is, it filters out any videos that were
    // ads. In the end, I used different logic because this is only one way that ads show up in the feed. What I
    // am using now is the existence of 3 anchors within that div.
    // Keeping the code in here though, because it took me some doing to figure it out. :-)
    // let all_elements
    //     = document.find(And(Class("outer-cell"), |node: &Node| !node.inner_html().contains(&"Google Ad")));

    {
        let all_elements = document.find(Class("outer-cell"));
        for node in all_elements {
            //       utils::print_subtree(&document, 0, node.index());

            let mut all_anchors = node.find(Name("a"));

            // entries with three anchors are the "real" ones - video, channel and link to settings.
            let three_anchors = all_anchors.next_chunk::<3>();

            if three_anchors.is_ok() {
                let anchors = three_anchors.ok().unwrap();
                let video_a = anchors[0];
                let video_url = video_a.attr("href").unwrap();
                let video_title = video_a.first_child().unwrap().as_text().unwrap();
                let channel_a = anchors[1];
                let channel_url = channel_a.attr("href").unwrap();
                let channel_title = channel_a.first_child().unwrap().as_text().unwrap();
                let date_str = channel_a.parent().unwrap().last_child().unwrap().text();
                let date_watched = NaiveDateTime::parse_from_str(
                    date_str.as_str().trim(),
                    "%b %e, %Y, %_I:%M:%S %p %Z",
                )
                .expect("Error parsing datetime");

                my_videos.push(Video { 
                    video_url : "a",
                    video_title : "a",
                    channel_url : "a",
                    channel_title : "a",
                    date_watched,
                });
            }
        }

        println!("{}", document.nodes.len());

    for i in 0..10 {
        println!("{:?}", my_videos[i]);
    }
    }
    (document, my_videos)
}
