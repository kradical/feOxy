use style::{StyledNode, Display};
use std::fmt;

#[derive(Default)]
struct Dimensions {
    content: Rect,
    padding: EdgeSizes,
    border: EdgeSizes,
    margin: EdgeSizes,
}

impl fmt::Debug for Dimensions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "content:\n  {:?}\npadding:\n  {:?}\nborder:\n  {:?}\nmargin:\n  {:?}",
            self.content, self.padding, self.border, self.margin)
    }
}

#[derive(Default)]
struct Rect {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl fmt::Debug for Rect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}, w: {}, h: {}", self.x, self.y, self.width, self.height)
    }
}

#[derive(Default)]
struct EdgeSizes {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
}

impl fmt::Debug for EdgeSizes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "l: {} r: {} top: {} bot: {}", self.left, self.right, self.top, self.bottom)
    }
}

pub struct LayoutBox {
    dimensions: Dimensions,
    box_type: BoxType,
    children: Vec<LayoutBox>,
}

impl LayoutBox {
    pub fn new(box_type: BoxType) -> LayoutBox {
        LayoutBox {
            box_type: box_type,
            dimensions: Default::default(),
            children: Vec::new(),
        }
    }

    fn get_inline(&mut self) -> &mut LayoutBox {
        match self.box_type {
            BoxType::Inline | BoxType::Anonymous => self,
            BoxType::Block => {
                match self.children.last() {
                    Some(&LayoutBox { box_type: BoxType::Anonymous, .. }) => {},
                    _ => self.children.push(LayoutBox::new(BoxType::Anonymous))
                }
                self.children.last_mut().unwrap()
            }
        }
    }
}

impl fmt::Debug for LayoutBox {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "type:\n  {:?}\n{:?}\n", self.box_type, self.dimensions)
    } 
}

pub enum BoxType {
    Block,
    Inline,
    Anonymous,
}

impl fmt::Debug for BoxType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display_type = match *self {
            BoxType::Block => "block",
            BoxType::Inline => "inline",
            BoxType::Anonymous => "anonymous"
        };

        write!(f, "{}", display_type)
    } 
}

fn build_layout_tree(node: &StyledNode) -> LayoutBox {
    let mut rect = LayoutBox::new(match node.get_display() {
        Display::Block => BoxType::Block,
        Display::Inline => BoxType::Inline,
        Display::None => panic!("root node has display: none")
    });

    for child in &node.children {
        match child.get_display() {
            Display::Block => rect.children.push(build_layout_tree(child)),
            Display::Inline => rect.get_inline().children.push(build_layout_tree(child)),
            Display::None => {}
        }
    }
    rect
}