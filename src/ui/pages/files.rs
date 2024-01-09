use crate::ui::{form::file_upload::file_upload_component, helpers::file::list_files};

pub fn files_page() -> String {
    let file_list = list_files();
    let file_upload_component = file_upload_component();
    let html: String = format!("
        <h1>About page</h1>
        <h2>Upload Files</h2>
        {}
        <h2>Here are all the files that you have uploaded to the site:</h2>
        {}        
    ", file_upload_component, file_list);
    html
}
