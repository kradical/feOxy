use dom::Node;

type PropertyMap = HashMap<String, String>

struct StyledNode<'a> {
    node: &'a Node,
    styles: PropertyMap,
    children: Vec<StyledNode<'a'>>
}

fn selector_matches(elem: &ElementData, selector: &Selector) -> bool {
    true
}