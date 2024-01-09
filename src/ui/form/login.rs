pub fn login_form() -> &'static str {
    let html: &str = r#"
            <form action="/login" method="post">
            <input id="username" name="username" type="text" placeholder="Username"/>
            <input id="password" name="password" type="password" placeholder="Password"/>
            <button type="submit">Login</button>
            </form>
        "#;
    html
}
