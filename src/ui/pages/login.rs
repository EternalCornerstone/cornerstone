use crate::ui::form;

pub fn login_page() -> String {
    let login_form = form::login::login_form();
    let html: String = format!("
            <h1>Login to the Server Core</h1>
            {}
            <a href='forgotten_password'>Forgot password?</a>
        ", login_form);
    html
}

