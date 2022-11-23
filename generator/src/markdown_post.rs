use chrono::{DateTime, FixedOffset};
use comrak::{self, ComrakOptions};
use walkdir::WalkDir;

pub fn markdown_to_html(markdown: &str) -> String {
    let mut options = ComrakOptions::default();
    options.render.unsafe_ = true;
    comrak::markdown_to_html(markdown, &options)
}

pub trait MarkdownPost: Sized {
    fn from_path(root_path: &str, path: &str) -> Option<Self>;
    fn publish_date(&self) -> DateTime<FixedOffset>;
}

pub fn relative_link(path: &str, root_path: &str) -> String {
    path[root_path.len()..]
        .rsplitn(2, '.')
        .last()
        .unwrap_or("")
        .to_string()
}

pub fn posts_from_root_path<Post: MarkdownPost>(root_path: &str, sub_path: &str) -> Vec<Post> {
    let root_path = format!(
        "{}/{}",
        if root_path.ends_with('/') {
            &root_path[0..root_path.len() - 1]
        } else {
            root_path
        },
        sub_path
    );

    let mut posts: Vec<Post> = WalkDir::new(&root_path)
        .into_iter()
        .flat_map(|e| e.ok())
        .filter(|e| {
            e.file_type().is_file()
                && e.file_name()
                    .to_str()
                    .map(|s| s.ends_with(".md"))
                    .unwrap_or(false)
        })
        .flat_map(|entry| {
            entry
                .path()
                .to_str()
                .map(|path| Post::from_path(&root_path, path))
        })
        .flatten()
        .collect();
    posts.sort_by(|a, b| b.publish_date().cmp(&a.publish_date()));
    posts
}
