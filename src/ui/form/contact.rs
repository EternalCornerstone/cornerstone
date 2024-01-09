pub fn contact_form() -> &'static str {
    let html: &str = r#"
            <form action="/contact" method="post">
            <div>
            <input id="name" name="name" type="text" placeholder="Name"/>
            </div>
            <div>
            <input id="email" name="email" type="email" placeholder="Email"/>
            </div>
            <div>
            <textarea id="message" name="message" type="text" placeholder="message" rows=4> </textarea>
            </div>
            <div>
            <button type="submit">Submit</button>
            </div>
            </form>
        "#;
    html
}
