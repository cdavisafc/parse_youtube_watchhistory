use select::document::Document;
use select::node::Data;

pub fn print_subtree(document: &Document, dep: usize, index: usize) {
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
