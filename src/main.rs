use std::collections::HashSet;

fn main() {
    let mut urls = HashSet::new();
    urls.insert("https://www.manning.com/books/rust-in-action");
    urls.insert("https://www.amazon.co.uk/Rust-Action-Tim-McNamara/dp/1617294551");

    let contains_reddit = urls.contains("https://www.reddit.com/r/rust/");

    println!("Does urls contain Reddit?\n {contains_reddit}");
}
