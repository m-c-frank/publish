use std::path::{Path};
use std::fs::create_dir_all;
use tokio::fs::{self, read_to_string, write};
use serde_yaml::Value;
use pulldown_cmark::{Parser, Options, html};

const PATH_NOTES: &str = "/home/mcfrank/publish/notes";
const PATH_OUTPUT: &str = "output";

async fn convert_markdown_to_html(markdown: &str) -> String {
    let parser = Parser::new_ext(markdown, Options::empty());
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

async fn make_index_page() -> Result<(), Box<dyn std::error::Error>> {
    let notes_dir = Path::new(PATH_NOTES);
    let output_dir = Path::new(PATH_OUTPUT);
    let output_file = output_dir.join("index.html");

    create_dir_all(&output_dir)?;

    let mut entries = fs::read_dir(&notes_dir).await?;
    let mut html_content = String::from("<!DOCTYPE html><html><head><title>Notes</title><link rel=\"stylesheet\" href=\"styles.css\"></head><body>");

    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "md") {
            let file_content = read_to_string(&path).await?;
            //log this to console
            println!("Converting file: {:?}", path);
            // make it relative to the notes directory
            let relative_path = path.strip_prefix(&notes_dir)?.to_str().unwrap();
            println!("Relative path: {:?}", relative_path);
            //now add back in the ./notes/ prefix
            let relative_path = format!("./notes/{}", relative_path);
            let (matter, content) = parse_front_matter(&file_content)?;
            let title = matter.get("title").and_then(|v| v.as_str()).unwrap_or_default();

            let note_title_for_index_page = format!("<a href=\"{}\">{}</a>", relative_path.replace(".md", ".html"), title);
            let note_title_for_single_page = format!("<h1>{}</h1>", title);

            let html = convert_markdown_to_html(&content).await;

            html_content.push_str(&format!("<div class=\"note\">{}{}</div>", note_title_for_index_page, html));

            let single_page_html = format!("<!DOCTYPE html><html><head><title>{}</title><link rel=\"stylesheet\" href=\"styles.css\"></head><body>{}{}</body></html>", title, note_title_for_single_page, html);

            let single_page_path = output_dir.join(relative_path.replace(".md", ".html"));
            if let Some(parent) = single_page_path.parent() {
                create_dir_all(parent)?;
            }
            write(&single_page_path, &single_page_html).await?;
        }
    }

    html_content.push_str("</body></html>");
    write(&output_file, &html_content).await?;

    println!("Conversion complete. Check the output folder for the output.");
    Ok(())
}

fn parse_front_matter(content: &str) -> Result<(Value, String), Box<dyn std::error::Error>> {
    let parts: Vec<&str> = content.splitn(3, "---").collect();
    if parts.len() != 3 {
        return Err("Invalid front matter format".into());
    }
    let matter: Value = serde_yaml::from_str(parts[1])?;
    Ok((matter, parts[2].to_string()))
}

#[tokio::main]
async fn main() {
    if let Err(e) = make_index_page().await {
        eprintln!("Error converting markdown to HTML: {:?}", e);
    }
}
