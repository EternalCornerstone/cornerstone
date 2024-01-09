use std::fs;

pub fn list_files() -> String {

    // Read the contents of the ./tmp directory
    let paths = fs::read_dir("./tmp").expect("Failed to read directory");

    // Create an HTML list item for each file
    let mut list_items = String::new();
    for path in paths {
        if let Ok(entry) = path {
            let path = entry.path();

            if let Some(file_name) = path.file_name() {
                if let Some(file_name_str) = file_name.to_str() {
                    // Get file metadata
                    let metadata = fs::metadata(&path).expect("Unable to read file metadata");
                    let file_size = metadata.len();

                    // Determine file type (extension)
                    let file_type = path.extension()
                        .and_then(|s| s.to_str())
                        .unwrap_or("unknown");

                    // Check if the file is an image
                    let is_image = matches!(file_type, "png" | "jpg" | "jpeg" | "gif");

                    // Generate HTML
                    if is_image {
                        list_items.push_str(&format!(
                            "<li><img src='./tmp/{}' alt='{}' style='width:100px; height:auto;'><br>{} ({} bytes, {})</li>\n",
                            file_name_str, file_name_str, file_name_str, file_size, file_type
                        ));
                    } else {
                        list_items.push_str(&format!(
                            "<li>{} ({} bytes, {})</li>\n",
                            file_name_str, file_size, file_type
                        ));
                    }
                }
            }
        }
    }


    let html = format!("
            <ul>
            {}
            <ul>
        ",  list_items);

        html
}