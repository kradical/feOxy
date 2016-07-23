mod dom;

fn main() {
    let mut body_attrs = dom::AttrMap::new();
    body_attrs.insert("class".to_string(), "red tall".to_string());
    body_attrs.insert("id".to_string(), "uniqueidentifier".to_string());
    body_attrs.insert("style".to_string(), ".red { background-color: red; }".to_string());

    let text1 = dom::text_node("I AM SOME TEXT TO BE DISPLAYED XD".to_string());
    let comment1 = dom::comment_node("shh i am sneky comment".to_string());
    let elem2 = dom::element_node("header".to_string(), dom::AttrMap::new(), Vec::new());
    let elem3 = dom::element_node("body".to_string(), body_attrs, vec![text1, comment1]);
    let elem1 = dom::element_node("html".to_string(), dom::AttrMap::new(), vec![elem2, elem3]);

    dom::pretty_print(elem1, 0);
}
