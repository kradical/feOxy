use style::StyledNode;
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

pub struct LayoutBox<'a> {
    dimensions: Dimensions,
    box_type: BoxType<'a>,
    children: Vec<LayoutBox<'a>>,
}

impl<'a> LayoutBox<'a> {
    pub fn new(box_type: BoxType) -> LayoutBox {
        LayoutBox {
            box_type: box_type,
            dimensions: Default::default(),
            children: Vec::new(),
        }
    }
}

impl<'a> fmt::Debug for LayoutBox<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "type:\n  {:?}\n{:?}\n", self.box_type, self.dimensions)
    } 
}

pub enum BoxType<'a> {
    Block(&'a StyledNode<'a>),
    Inline(&'a StyledNode<'a>),
    Anonymous,
}

impl<'a> fmt::Debug for BoxType<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display_type = match *self {
            BoxType::Block(_) => "block",
            BoxType::Inline(_) => "inline",
            BoxType::Anonymous => "anonymous"
        };

        write!(f, "{}", display_type)
    } 
}