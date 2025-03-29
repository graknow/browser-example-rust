pub fn show(html: &str) {
    let mut in_tag: bool = false;
    for c in html.chars() {
        if c == '<' {
            in_tag = true;
        } else if c == '>' {
            in_tag = false;
        } else if !in_tag {
            print!("{}", c);
        }
    }
}
