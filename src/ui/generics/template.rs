use super::navigation::navigation_bar;

pub fn return_template(page: String) -> String {
    let navigation_bar = navigation_bar(false, false); //logged in and admin.
    // let nav_html = navigation_bar(true, false); // User is logged in, but not an admin.
    // let nav_html = navigation_bar(false, false); // User is not logged in, obviously cannot be admin.
    let html: String = format!("<html  >
            <head><title>Core Server</title></head>
            <body>
                {}
                <h1>Welcome To The Website.</h1>
                {}
            </body>
            
        </html>", navigation_bar, page
    );
    html
}