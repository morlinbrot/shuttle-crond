pub async fn qotw() {
    let owner = "rust-lang";
    let repo = "this-week-in-rust";

    let filename = "2023-05-24-this-week-in-rust.md";
    let path = format!("content/{filename}");

    // TODO: Query for directory, grab last entry.
    let mut content = octocrab::instance()
        .repos(owner, repo)
        .get_content()
        .path(path)
        .r#ref("master")
        .send()
        .await
        .unwrap();

    let contents = content.take_items();
    let c = &contents[0];
    let content = c.decoded_content().unwrap();

    let lines = content
        .split('\n')
        .filter(|l| l.starts_with("# Quote of the Week") || l.starts_with(">"));

    let quote = lines
        .skip(1)
        .map(|l| l.trim_start_matches(&['>', ' ']))
        .collect::<Vec<_>>()
        .join("\n");

    print!("{quote}");
}

// pub async fn connect() {}
