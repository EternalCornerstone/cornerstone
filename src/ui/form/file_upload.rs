pub fn file_upload_component() -> &'static str {
    let html: &str = r#"<form target="/" method="post" enctype="multipart/form-data">
        <input type="file" multiple name="file"/>
        <button type="submit">Submit</button>
    </form>"#;
    html
}