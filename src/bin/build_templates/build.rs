use crate::md2html;
use itertools::Itertools;
use log::error;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct BlogInfo {
    short: String,
    title: String,
    date: String,
}

pub fn build_blog() {
    let mut blog_info_list: Vec<BlogInfo> = Vec::new();

    let blog_template = match fs::read_to_string("./templates/blog_post.tmpl") {
        Ok(t) => t,
        Err(e) => panic!("unable to open blog template: {}", e.to_string()),
    };

    for blog_post in fs::read_dir("./blog").unwrap().flatten() {
        let blog_md = match fs::read_to_string(blog_post.path()) {
            Ok(m) => m,
            Err(e) => {
                error!("unable to read blog file: {}", e.to_string());
                continue;
            }
        };

        let mut blog_header = String::new();

        let mut lines = blog_md.lines();

        for _ in 1..=3 {
            let line = lines.next().unwrap();
            blog_header.push_str(line);
            blog_header.push('\n');
        }

        let file = lines.join("\n");

        let blog_info: BlogInfo = match serde_yaml::from_str(blog_header.as_str()) {
            Ok(b) => b,
            Err(e) => {
                error!("unable to serialize header: {}", e.to_string());
                continue;
            }
        };

        let filename = format!("./static/blog/{}.html", blog_info.short);

        let article_html = md2html::convert(file.as_str());

        let formatted_post = blog_template
            .replace("{{post_title}}", blog_info.title.as_str())
            .replace("{{post_date}}", blog_info.date.as_str())
            .replace("{{post_article}}", article_html.as_str());

        fs::write(filename, formatted_post).unwrap();

        blog_info_list.push(blog_info);
    }
    build_index(blog_info_list)
}

fn build_index(blog_posts: Vec<BlogInfo>) {
    let index_template = match fs::read_to_string("./templates/index.tmpl") {
        Ok(i) => i,
        Err(e) => {
            panic!("unable to read index template: {}", e);
        }
    };

    let mut post_html: String = String::new();

    for post in blog_posts {
        post_html += &*format!(
            "<div class=\"article\">\
                <span class=\"date\">{}</span>\
                <a href=\"/blog/{}\">{}</a>\
            </div>\n",
            post.date, post.short, post.title
        )
    }

    let formatted_index = index_template.replace("{{ARTICLES}}", post_html.as_str());

    fs::write("./static/index.html", formatted_index).expect("unable to write index file");
}
