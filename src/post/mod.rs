use std::path::{Path, PathBuf};
use super::frontmatter::{RawFrontMatter, FrontMatter};
use tera::Tera;

mod katex;
mod markdown;
mod plantuml;
mod pygments;

lazy_static::lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec!["html"]);
        tera
    };
}

pub struct Post {
    pub front: FrontMatter,
    pub source: PathBuf,
    pub contents: String,
}

impl Post {
    fn extract_frontmatter(src: &str) -> Result<(Option<RawFrontMatter>, String), Box<dyn std::error::Error>> {
        if src.starts_with("---\n") {
            let slice = &src[4..];
            let end = slice.find("---\n");
            if end.is_none() {
                return Ok((None, src.to_owned()));
            }
            let end = end.unwrap();
    
            let front = &slice[..end];
            let contents = &slice[end+4..];
            let front: RawFrontMatter = serde_yaml::from_str(front)?;
            Ok((Some(front), contents.to_owned()))
        }
        else if src.starts_with("---\r\n") {
            let slice = &src[5..];
            let end = slice.find("---\r\n");
            if end.is_none() {
                return Ok((None, src.to_owned()));
            }
            let end = end.unwrap();
    
            let front = &slice[..end];
            let contents = &slice[end+5..];
            let front: RawFrontMatter = serde_yaml::from_str(front)?;
            Ok((Some(front), contents.to_owned()))
        }
        else {
            Ok((None, src.to_owned()))
        }
    }

    pub fn load<P: AsRef<Path>>(src: P) -> Result<Option<Post>, Box<dyn std::error::Error>> {
        let contents = std::fs::read_to_string(src.as_ref())?;

        let (front, contents) = Post::extract_frontmatter(&contents)?;
        if front.is_none() {
            eprintln!("skipping `{}` as it contains invalid metadata", src.as_ref().display());
            return Ok(None);
        }

        let front: Option<FrontMatter> = front.unwrap().into();
        if front.is_none() {
            eprintln!("skipping `{}` as it isn't published", src.as_ref().display());
            return Ok(None);
        }
        let front = front.unwrap();

        Ok(Some(Post {
            front,
            contents,
            source: src.as_ref().to_owned(),
        }))
    }

    pub fn render(&self) -> Result<String, Box<dyn std::error::Error>> {
        let markdown::FormatResponse { output, include_katex_css } = markdown::format_markdown(&self.contents)?;

        let mut context = tera::Context::new();
        context.insert("title", &self.front.title);
        context.insert("content", &output);
        context.insert("include_katex_css", &include_katex_css);

        let rendered = TEMPLATES.render("post.html", &context)?;

        Ok(rendered)
    }
}
