pub fn registration_form() -> &'static str {
    let html: &str = r#"
        <form action="/register" method="post">
            <input id="username" name="username" type="text" placeholder="Username"/>
            <input id="email" name="email" type="email" placeholder="Email"/>
            <input id="password" name="password" type="password" placeholder="Password"/>
            <button type="submit">Register</button>
        </form>
    "#;
    html
}