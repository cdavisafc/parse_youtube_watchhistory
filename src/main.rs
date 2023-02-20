
#![feature(iter_next_chunk)]

use select::document::{Document, self};
use select::node::{Data, Raw};
use select::predicate::{Attr, Class, Name, Predicate};
use chrono::{DateTime, NaiveDateTime, FixedOffset};
use std::iter::OnceWith;
use std::mem;

#[derive(Debug)]
pub struct Video<'a> {
    pub video_url : &'a str,
    pub video_title : &'a str,
    pub channel_url : &'a str,
    pub channel_title : &'a str,
    pub date_str : String,
    pub date_watched : NaiveDateTime,
}
fn main() {

    let mut my_videos: Vec<Video> = vec![];

    let document = Document::from(include_str!("../watch-history.html"));
    println!("loaded doc {}", document.nodes.len());

    let all_elements = document.find(Class("outer-cell"));
    for node in all_elements {

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
            let date_watched = NaiveDateTime::parse_from_str(date_str.as_str().trim(), "%b %e, %Y, %_I:%M:%S %p %Z").expect("Error parsing datetime");

            my_videos.push(Video {
                video_url,
                video_title,
                channel_url,
                channel_title,
                date_str,
                date_watched,
            });

        }
    }
     
    for i in 0..10 {
        println!("{:?}", my_videos[i]);
    }

    fn print_subtree(document: &Document, dep: usize, index: usize) {
        for i in 0..dep {
            print!("  ");
        }
        println!("{:p} {:?}", &(document.nodes[index]), document.nodes[index]);
        match document.nodes[index].data {
            Data::Element (_, _) => {
                let mut childi = document.nodes[index].first_child;
                while childi.is_some() {
                    print_subtree(document, dep+1, childi.unwrap());
                    childi = document.nodes[childi.unwrap()].next;
                }
            }
            _ => (),
        }
    }
}
