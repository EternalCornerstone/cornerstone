pub fn forgotten_password() -> &'static str {
    let html: &str = r#"
            <form action="/forgotten_password" method="post">
                <input id="email" name="email" type="email" placeholder="Email"/>
                <button type="submit">Login</button>
            </form>
        "#;
    html
}
