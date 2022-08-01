struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    }
    else {
        y
    }
}

fn main() {
    let novel = String::from("sentance one. sentance two. sentance three.");
    let first_sentence = novel.split('.').next().expect("Could not find .");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
