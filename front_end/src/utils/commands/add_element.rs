use web_sys::{window, Element, Node};

pub fn add_script() -> Element {
    let window = window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();
    let script = document.create_element("script").unwrap();

    //script should be just alert("Hello world")
    script.set_inner_html(&format!(
        r#"
    alert("Hello")
    "#
    ));
    // and append it to the body
    body.append_child(&script).unwrap();

    return script;
}

pub fn add_loading() -> Node {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let loading = document.create_element("p").unwrap();
    loading.set_class_name("animate-pulse");
    loading.set_text_content(Some("Loading..."));
    let elements = document.get_elements_by_name("raw_html");
    let element = elements.item(elements.length() - 1).unwrap();
    let child = element.append_child(&loading).unwrap();
    return child;
}

pub fn remove_loading(loading: Node) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let elements = document.get_elements_by_name("raw_html");
    let element = elements.item(elements.length() - 1).unwrap();
    element.remove_child(&loading).unwrap();
}
