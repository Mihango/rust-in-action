fn main() {
    let search_item = "picture";
    let quote = "\
    Every face, every shop, bedroom window, public-house, and
 dark square is a picture feverishly turned--in search of what?
 It is the same with books.
 What do we seek through millions of pages?
    ";

    for (line_num, line) in quote.lines().enumerate() {
        if line.contains(search_item) {
            println!("{}:{}", line_num + 1, line);
        }
    }
}
