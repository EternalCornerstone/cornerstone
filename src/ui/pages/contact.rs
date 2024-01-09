use crate::ui::form::contact::contact_form;

pub fn contact_page() -> String {
    let contact_form = contact_form();
    let html: String = format!("
            <h1>Contact page</h1>
            <h2>Let us know if you have any questions or ideas.</h2>
            {}
            <p>Any cool ideas or any helpful tips will be greatly appreciated. Bug reports are also greatly wanted.</p> 
        ", contact_form);
    html
}