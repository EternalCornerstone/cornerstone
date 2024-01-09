pub fn navigation_bar(is_logged_in: bool, is_admin: bool) -> String {
    let mut nav_links = vec![
        "<a href=\"/\">Home</a>",
        "<a href=\"/about\">About</a>",
        "<a href=\"/contact\">Contact</a>",
        "<a href=\"/products\">Products</a>",
    ];

    if is_logged_in {
        nav_links.push("<a href=\"/profile\">Profile</a>");
        // nav_links.push("<a href=\"/file_upload\">Upload File</a>");
        if is_admin {
            nav_links.push("<a href=\"/cms\">CMS</a>");
        }
        nav_links.push("<a href=\"/logout\">Logout</a>");
    } else {
        nav_links.push("<a href=\"/login\">Login</a>");
        nav_links.push("<a href=\"/register\">Register</a>");
    }

    let nav_html = nav_links.join(" | ");
    format!("<nav>{}</nav>", nav_html)
}