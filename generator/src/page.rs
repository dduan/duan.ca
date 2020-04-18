use std::fs;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug, PartialEq)]
pub struct Page {
    pub relative_link: String,
    pub title: String,
}

impl Page {
    pub fn read_body(&self, root_path: &str) -> Option<String> {
        fs::read_to_string(format!("{}/pages{}{}.html", root_path, self.relative_link, self.title)).ok()
    }

    fn from_path(root_path: &str, path: &str) -> Option<Page> {
        let path = Path::new(path);
        if let Some(title) = Page::title_from_path(&path) {
            if let Some(relative_link) = Page::relative_link_from_path(root_path, path) {
                return Some(Page {
                    relative_link: relative_link,
                    title: title,
                })
            }
        }

        None
    }

    fn title_from_path(path: &Path) -> Option<String> {
        path
            .file_stem()
            .and_then(|s| s.to_str())
            .map (|s| s.to_string())
    }

    fn relative_link_from_path(root_path: &str, path: &Path) -> Option<String> {
        path
            .parent()
            .and_then(|p| p.strip_prefix(root_path).ok())
            .and_then(|s| s.to_str())
            .map (|s| format!("/{}", s))
    }
}

// TODO: de-duplicate this logic with `article::articles_from_root_path` with a trait!
pub fn pages_from_root_path(root_path: &str) -> Vec<Page> {
    let root_path = format!(
        "{}/pages",
        if root_path.ends_with("/") {
            &root_path[0..root_path.len() - 1]
        } else {
            root_path
        });

    WalkDir::new(&root_path)
        .into_iter()
        .flat_map(|e| e.ok())
        .filter(|e| {
            e.file_type().is_file() &&
                e.file_name()
                .to_str()
                .map(|s| s.ends_with(".html"))
                .unwrap_or(false)

        })
    .flat_map(|entry| {
        entry
            .path()
            .to_str()
            .map(|path| Page::from_path(&root_path, path))
    })
    .flat_map(|a| a)
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn title_from_path() {
        let title = "test";
        let path_str = format!("/root/pages/{}.html", title);
        let path = Path::new(&path_str);
        assert_eq!(Page::title_from_path(&path), Some(title.to_string()))
    }

    #[test]
    fn relative_link_from_path() {
        let relative_link = "/about";
        let root = "/root/pages";
        let path_str = format!("{}{}/test.html", root, relative_link);
        let path = Path::new(&path_str);
        assert_eq!(Page::relative_link_from_path(root, &path), Some(relative_link.to_string()));
    }
}
