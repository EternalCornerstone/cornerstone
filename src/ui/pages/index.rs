pub fn index_page() -> String {
    let html: String = format!("
            <h1>Home page</h1>
            <h2>This site will be a barebones SSR for a basic website.</h2>
            <p>This includes, Auth/Files/Account Subscriptions (payment system)/Basic CMS to edit profile / file data.</p> 
            <p>Basic read/write interface to database tables</p>
        ");
    html
}