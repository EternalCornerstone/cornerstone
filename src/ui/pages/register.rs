use crate::ui::form::register::registration_form;

pub fn register_page() -> String {
    // eventually registration_form(url, theme, blah) ??
    let registration_form = registration_form();
    let html: String = format!("<h1>Register your account!</h1> {}", registration_form);
    html
}
