use crate::ui::form;


pub fn forgotten_password_page() -> String {
    let forgotten_password_form = form::forgotten_password::forgotten_password();

    let html: String = format!("
    <h2>Oops. You forgot your password!</h2>
    <h3>Not to worry, if you have an email associated with your account then we can send a recovery email.</h3>
    {}
", forgotten_password_form);
html
}
