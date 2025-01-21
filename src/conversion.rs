//export convert function to main.rs
pub fn convert(markdown: &str) -> String {
    let mut html = String::new();
    let mut in_list = false;
    let mut in_list_item = false;
    let mut in_paragraph = false;

    for line in markdown.lines() {
        if line.starts_with("######") {
            html.push_str(&format!("<h6>{}</h6>", &line[7..]));
        } else if line.starts_with("#####") {
            html.push_str(&format!("<h5>{}</h5>", &line[6..]));
        } else if line.starts_with("####") {
            html.push_str(&format!("<h4>{}</h4>", &line[5..]));
        } else if line.starts_with("###") {
            html.push_str(&format!("<h3>{}</h3>", &line[4..]));
        } else if line.starts_with("##") {
            html.push_str(&format!("<h2>{}</h2>", &line[3..]));
        } else if line.starts_with("#") {
            html.push_str(&format!("<h1>{}</h1>", &line[2..]));
        } else if line.starts_with("- ") {
            if !in_list {
                html.push_str("<ul>");
                in_list = true;
            }
            if in_list_item {
                html.push_str("</li>");
            }
            html.push_str(&format!("<li>{}</li>", &line[2..]));
            in_list_item = true;
        } else if line.starts_with("1. ") {
            if !in_list {
                html.push_str("<ol>");
                in_list = true;
            }
            if in_list_item {
                html.push_str("</li>");
            }
            html.push_str(&format!("<li>{}</li>", &line[3..]));
            in_list_item = true;
        } else {
            if in_list_item {
                html.push_str("</li>");
                in_list_item = false;
            }
            if in_list {
                html.push_str(if line.starts_with("1. ") { "</ol>" } else { "</ul>" });
                in_list = false;
            }
            if !in_paragraph {
                html.push_str("<p>");
                in_paragraph = true;
            }
            html.push_str(&format!("{}<br>", &line));
        }
    }

    // Close any open tags at the end of the document
    if in_list_item {
        html.push_str("</li>");
    }
    if in_list {
        html.push_str(if markdown.lines().any(|line| line.starts_with("1. ")) { "</ol>" } else { "</ul>" });
    }
    if in_paragraph {
        html.push_str("</p>");
    }

    html
}
