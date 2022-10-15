use pulldown_cmark::{html, Options, Parser};

pub fn convert(md: &str) -> String {
    let options = Options::empty();
    let parser = Parser::new_ext(md, options);

    let mut html_output: String = String::with_capacity(md.len() * 3 / 2);

    html::push_html(&mut html_output, parser);

    html_output
}
