mod article;
mod page;
mod templates;
use std::env;
use article::Article;
use page::Page;
use templates::{ArticleTemplate, RenderedTag, RenderedMetadata};
use askama::Template;

/*
use comrak::{self, ComrakOptions};
use std::path::Path;
use syntect::highlighting::ThemeSet;
use syntect::html::css_for_theme;
use std::io::{BufWriter, Write};
*/

#[derive(Debug, PartialEq)]
struct Site {
    base_url: String,
    articles: Vec<Article>,
    pages: Vec<Page>,
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let root_path = args[1].to_string();
    let content = Site {
        base_url: "http://localhost:8000".to_string(),
        articles: article::articles_from_root_path(&root_path),
        pages: page::pages_from_root_path(&root_path),
    };

    for a in content.articles {
        println!("{:?}", a);
        println!("{:?}", a.read_body(&root_path));
    }

    for p in content.pages {
        println!("{:?}", p);
        println!("{:?}", p.read_body(&root_path));
    }

    let hello = ArticleTemplate {
        meta: RenderedMetadata { permalink: "https://localhost/hello", title: "My Post", },
        current_url: "/hello",
        date: "2020-04-17",
        content: "A good day without quarintine",
        tags: vec![&RenderedTag { name: "Rust", slug: "rust" }, &RenderedTag { name: "Swift", slug: "swift" }]
    };
    println!("{}", hello.render().unwrap()); // then render it.
    //let markdown = fs::read_to_string(&args[1]).unwrap();
    //let html = comrak::markdown_to_html(&markdown, &ComrakOptions::default());

    /*
    // ---------------------------------------------------------------------------------------------
    // generate html
    let ss = SyntaxSet::load_defaults_newlines();

    let html_file =  File::create(Path::new("synhtml-css-classes.html"))?;
    let mut html = BufWriter::new(&html_file);

    // write html header
    writeln!(html, "<!DOCTYPE html>")?;
    writeln!(html, "<html>")?;
    writeln!(html, "  <head>")?;
    writeln!(html, "    <title>synhtml-css-classes.rs</title>")?;
    writeln!(html, "    <link rel=\"stylesheet\" href=\"synhtml-css-classes.css\">")?;
    writeln!(html, "  </head>")?;
    writeln!(html, "  <body>")?;

    // Rust
    let code_rs = "// Rust source
fn main() {
    println!(\"Hello World!\");
}";

    let sr_rs = ss.find_syntax_by_extension("rs").unwrap();
    let mut rs_html_generator = ClassedHTMLGenerator::new(&sr_rs, &ss);
    for line in code_rs.lines() {
        rs_html_generator.parse_html_for_line(&line);
    }
    let html_rs = rs_html_generator.finalize();

    writeln!(html, "<pre class=\"code\">")?;
    writeln!(html, "{}", html_rs)?;
    writeln!(html, "</pre>")?;

    // C++
    let code_cpp = "/* C++ source */
#include <iostream>
int main() {
    std::cout << \"Hello World!\" << std::endl;
}";

    let sr_cpp = ss.find_syntax_by_extension("cpp").unwrap();
    let mut cpp_html_generator = ClassedHTMLGenerator::new(&sr_cpp, &ss);
    for line in code_cpp.lines() {
        cpp_html_generator.parse_html_for_line(&line);
    }
    let html_cpp = cpp_html_generator.finalize();

    writeln!(html, "<pre class=\"code\">")?;
    writeln!(html, "{}", html_cpp)?;
    writeln!(html, "</pre>")?;

    // write html end
    writeln!(html, "  </body>")?;
    writeln!(html, "</html>")?;

    // ---------------------------------------------------------------------------------------------
    // generate css
    let css = "@import url(\"theme-light.css\") (prefers-color-scheme: light);
    @import url(\"theme-dark.css\") (prefers-color-scheme: dark);
    @media (prefers-color-scheme: dark) {
      body {
        background-color: gray;
      }
    }
    @media (prefers-color-scheme: light) {
      body {
        background-color: lightgray;
      }
    }";

    let css_file = File::create(Path::new("synhtml-css-classes.css"))?;
    let mut css_writer = BufWriter::new(&css_file);

    writeln!(css_writer, "{}", css)?;

    // ---------------------------------------------------------------------------------------------
    // generate css files for themes
    let ts = ThemeSet::load_defaults();

    // create dark color scheme css
    let dark_theme = &ts.themes["Solarized (dark)"];
    let css_dark_file = File::create(Path::new("theme-dark.css"))?;
    let mut css_dark_writer = BufWriter::new(&css_dark_file);

    let css_dark = css_for_theme(dark_theme);
    writeln!(css_dark_writer, "{}", css_dark)?;

    // create light color scheme css
    let light_theme = &ts.themes["Solarized (light)"];
    let css_light_file = File::create(Path::new("theme-light.css"))?;
    let mut css_light_writer = BufWriter::new(&css_light_file);

    let css_light = css_for_theme(light_theme);
    writeln!(css_light_writer, "{}", css_light)?;
    */

    Ok(())
}

