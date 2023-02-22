#![feature(iter_next_chunk)]

use chrono::NaiveDateTime;
use scraper::{Html, Selector};
use std::fs;

#[derive(Debug)]
pub struct Video {
    pub video_url: String,
    pub video_title: String,
    pub channel_url: String,
    pub channel_title: String,
    pub date_watched: NaiveDateTime,
}

fn main() {
    let mut my_videos: Vec<Video> = vec![];

    let document = Html::parse_document(fs::read_to_string("watch-history.html").unwrap().as_str());

    let all_elements_selector = Selector::parse("div.outer-cell").unwrap();
    let all_elements = document.select(&all_elements_selector);
    // println!("loaded doc - num nodes {}", all_elements.count());  //scraper::Html is an iterator so cannot consume it here and use it later.

    // Okay, so I am finding the selector use here at least as clunky as that of select. I do think the more complex 
    // query will be cleaner.
    // Update: I actually don't think so. It looks like CSS does not have such a selector.
    // let all_elements_selector = Selector::parse("div.outer-cell <and what I want here is a way to say the inner_html contains a certain string>").unwrap();
    // let all_elements = document.select(&all_elements_selector);

    for node in all_elements {
        let three_anchors = node.select(&Selector::parse("a").unwrap()).next_chunk::<3>();

        if three_anchors.is_ok() {
            let anchors = three_anchors.ok().unwrap();
            let video_a = anchors[0];
            let video_url = video_a.value().attr("href").unwrap().to_string();
            let video_title = video_a.inner_html().to_string();
            let channel_a = anchors[1];
            let channel_url = channel_a.value().attr("href").unwrap().to_string();
            let channel_title = channel_a.inner_html().to_string();
            let date_str = channel_a.parent().unwrap().last_child().unwrap().value().as_text().unwrap().to_string();
            let date_watched = NaiveDateTime::parse_from_str(
                date_str.as_str().trim(),
                "%b %e, %Y, %_I:%M:%S %p %Z",
            )
            .expect("Error parsing datetime");

            my_videos.push(Video {
                video_url,
                video_title,
                channel_url,
                channel_title,
                date_watched,
            });
        }
    }

    for i in 0..10 {
        println!("{:?}", my_videos[i]);
    }

    // fn print_subtree(document: &Document, dep: usize, index: usize) {
    //     for i in 0..dep {
    //         print!("  ");
    //     }
    //     println!("{:p} {:?}", &(document.nodes[index]), document.nodes[index]);
    //     match document.nodes[index].data {
    //         Data::Element (_, _) => {
    //             let mut childi = document.nodes[index].first_child;
    //             while childi.is_some() {
    //                 print_subtree(document, dep+1, childi.unwrap());
    //                 childi = document.nodes[childi.unwrap()].next;
    //             }
    //         }
    //         _ => (),
    //     }
    // }
}
