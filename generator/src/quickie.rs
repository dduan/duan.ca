use crate::markdown_post::{relative_link, MarkdownPost};
use chrono::{DateTime, FixedOffset};
use std::{fs, str};

#[derive(Debug, PartialEq)]
pub struct Quickie {
    pub relative_link: String,
    pub content: String,
    pub date: DateTime<FixedOffset>,
    pub media_links: Vec<String>,
    pub syndicate_links: Vec<String>,
    pub related_link: Option<String>,
}

impl Quickie {
    fn from_str(relative_link: String, content: &str) -> Option<Self> {
        let parts = content.split("\n---\n\n").collect::<Vec<&str>>();
        let len = parts.len();
        if len < 1 || len > 4 {
            Option::None
        } else {
            date_and_body(&parts[0]).map(|(date, content)| {
                let media_links: Vec<String> = if len > 1 {
                    parts[1].lines().map(|x| x.to_string()).collect()
                } else {
                    Vec::new()
                };

                let syndicate_links: Vec<String> = if len > 2 {
                    parts[2].lines().map(|x| x.to_string()).collect()
                } else {
                    Vec::new()
                };

                let related_link: Option<String> = if len > 3 {
                    Some(strip_newline(parts[3]).to_string())
                } else {
                    None
                };

                Quickie {
                    relative_link,
                    content,
                    date,
                    media_links,
                    syndicate_links,
                    related_link,
                }
            })
        }
    }
}

fn strip_newline<'a>(line: &'a str) -> &'a str {
    line.strip_suffix('\n').unwrap_or(line)
}

fn date_and_body(text: &str) -> Option<(DateTime<FixedOffset>, String)> {
    let parts: Vec<&str> = text.splitn(2, "\n\n").into_iter().collect();
    if parts.len() < 1 {
        None
    } else {
        DateTime::parse_from_rfc3339(parts[0]).ok().map(|date| {
            (
                date,
                if parts.len() < 2 {
                    ""
                } else {
                    strip_newline(parts[1])
                }
                .to_string(),
            )
        })
    }
}

impl MarkdownPost for Quickie {
    fn from_path(root_path: &str, path: &str) -> Option<Self> {
        let link = format!("/quickie{}", relative_link(path, root_path));
        fs::read_to_string(path)
            .ok()
            .and_then(|content| Quickie::from_str(link, &content))
    }

    fn publish_date(&self) -> DateTime<FixedOffset> {
        self.date
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn date_and_body_from_string() {
        let s = r#"2022-11-22T16:38:27-08:00

Test: this is a micro post.
"#;
        assert_eq!(
            Some((
                DateTime::parse_from_rfc3339("2022-11-22T16:38:27-08:00").unwrap(),
                "Test: this is a micro post.".to_string()
            )),
            date_and_body(s)
        );
    }

    #[test]
    fn quickie_from_string() {
        let s = r#"2022-11-22T16:38:27-08:00

Test: this is a micro post.

---

/assets/2020/12/disk-utility.png
https://avatars.githubusercontent.com/u/75067

---

https://twitter.com/daniel_duan/status/1595200469932310530

---

https://twitter.com/daniel_duan/status/1595197898068615169
"#;
        assert_eq!(
            Some(Quickie {
                relative_link: "lol".to_string(),
                content: "Test: this is a micro post.".to_string(),
                date: DateTime::parse_from_rfc3339("2022-11-22T16:38:27-08:00").unwrap(),
                media_links: vec![
                    "/assets/2020/12/disk-utility.png".to_string(),
                    "https://avatars.githubusercontent.com/u/75067".to_string(),
                ],
                syndicate_links: vec![
                    "https://twitter.com/daniel_duan/status/1595200469932310530".to_string()
                ],
                related_link: Some(
                    "https://twitter.com/daniel_duan/status/1595197898068615169".to_string()
                ),
            }),
            Quickie::from_str("lol".to_string(), s)
        )
    }
}
