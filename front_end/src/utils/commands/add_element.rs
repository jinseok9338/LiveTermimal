use web_sys::Node;

pub fn add_loading() -> Node {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let loading = document.create_element("p").unwrap();
    loading.set_class_name("animate-pulse");
    // set style to color lime
    loading.set_attribute("style", "color: lime").unwrap();
    loading.set_text_content(Some("Loading..."));
    let elements = document.get_elements_by_name("raw_html");

    // if elements is empty, create div whose name is raw_html
    if elements.length() == 0 {
        let history_list = document
            .get_elements_by_name("history_list")
            .item(0)
            .unwrap();
        let child = history_list.append_child(&loading).unwrap();

        return child;
    }
    let element = elements.item(elements.length() - 1).unwrap();
    element.append_child(&loading).unwrap()
}

pub fn remove_loading(loading: Node) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let elements = document.get_elements_by_name("raw_html");

    if elements.length() == 0 {
        let history_list = document
            .get_elements_by_name("history_list")
            .item(0)
            .unwrap();
        history_list.remove_child(&loading).unwrap();
        return;
    }

    let element = elements.item(elements.length() - 1).unwrap();
    element.remove_child(&loading).unwrap();
}
