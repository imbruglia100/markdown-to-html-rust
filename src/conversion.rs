//export convert function to main.rs
pub fn convert(markdown: &str) -> String {
    let mut html = String::new();
    let mut in_list = false;
    let mut in_list_item = false;
    let mut in_paragraph = false;
    let mut _in_emphasis = false;
    let mut _in_strong = false;

    for line in markdown.lines() {
        if line.starts_with("#") {
            html.push_str(&format!("<h1>{}</h1>", &line[2..]));
        } else if line.starts_with("##") {
            html.push_str(&format!("<h2>{}</h2>", &line[3..]));
        } else if line.starts_with("###") {
            html.push_str(&format!("<h3>{}</h3>", &line[4..]));
        } else if line.starts_with("####") {
            html.push_str(&format!("<h4>{}</h4>", &line[5..]));
        } else if line.starts_with("#####") {
            html.push_str(&format!("<h5>{}</h5>", &line[6..]));
        } else if line.starts_with("######") {
            html.push_str(&format!("<h6>{}</h6>", &line[7..]));
        } else if line.starts_with("- ") {
            if !in_list {
                html.push_str("<ul>");
                in_list = true;
            }
            if !in_list_item {
                html.push_str("<li>");
                in_list_item = true;
            }
            html.push_str(&format!("{}", &line[2..]));
        } else {
            if in_list_item {
                html.push_str("</li>");
                in_list_item = false;
            }
            if in_list {
                html.push_str("</ul>");
                in_list = false;
            }
            if !in_paragraph {
                html.push_str("<p>");
                in_paragraph = true;
            }
            html.push_str(&format!("{}<br>", &line));
        }
    }
    println!("{}", html);
    return html.to_string();
}
