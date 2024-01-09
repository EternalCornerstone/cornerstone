pub fn error_page() -> &'static str {
    let html: &str = r#"<h1>Error: 404</h1> 
    <h3>Not Found</h3>"#;
    html
}
