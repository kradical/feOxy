use style::{StyledNode, Display};
use std::fmt;

#[derive(Clone, Copy, Default)]
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

#[derive(Clone, Copy, Default)]
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

#[derive(Clone, Copy, Default)]
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

    fn get_inline(&mut self) -> &mut LayoutBox<'a> {
        match self.box_type {
            BoxType::Inline(_) | BoxType::Anonymous => self,
            BoxType::Block(_) => {
                match self.children.last() {
                    Some(&LayoutBox { box_type: BoxType::Anonymous, .. }) => {},
                    _ => self.children.push(LayoutBox::new(BoxType::Anonymous))
                }
                self.children.last_mut().unwrap()
            }
        }
    }

    fn layout(&mut self, bounding_box: Dimensions) {
        match self.box_type {
            BoxType::Block(_) => self.layout_block(bounding_box),
            BoxType::Inline(_) => {},
            BoxType::Anonymous => {},
        }
    }

    fn layout_block(&mut self, b_box: Dimensions)  {
        self.calculate_width(b_box);
        self.calculate_position(b_box);
        self.layout_children();
        self.calculate_height()
    }

    fn calculate_width(&mut self, b_box: Dimensions) {
        let style = self.get_style_node();
        let mut width: f32 = style.value_or("width", 0.0);
        let mut margin_l: f32 = style.value_or("margin-left", 0.0);
        let mut margin_r: f32 = style.value_or("margin-right", 0.0);
        let border_l: f32 = style.value_or("border-left-width", 0.0);
        let border_r: f32 = style.value_or("border-right-width", 0.0);
        let padding_l: f32 = style.value_or("padding-left", 0.0);
        let padding_r: f32 = style.value_or("padding-right", 0.0);

        let total = width + margin_l + margin_r + border_l + border_r + padding_l + padding_r;

        let underflow = b_box.content.width - total;

        match (width, margin_l, margin_r) {
            // width is auto
            (0.0, _, _) => {
                if underflow >= 0.0 {
                    width = underflow;
                } else {
                    // width can't be negative
                    margin_r = margin_r + underflow;
                }
            },
            // left margin is auto
            (w, 0.0, mr) if w != 0.0 && mr != 0.0 => { margin_l = underflow; },
            // right margin is auto
            (w, ml, 0.0) if w != 0.0 && ml != 0.0 => { margin_r = underflow; },
            // left/right margin are auto
            (w, 0.0, 0.0) if w != 0.0 => {
                margin_l = underflow / 2.0;
                margin_r = underflow / 2.0;
            },
            // values are overconstrained, calculate margin_right.
            (_, _, _) => { margin_r = margin_r + underflow; },
        }

        let d = &mut self.dimensions;
        d.content.width = width;
        d.padding.left = padding_l;
        d.padding.right = padding_r;
        d.border.left =  border_l;
        d.border.right = border_r;
        d.margin.left =  margin_l;
        d.margin.right = margin_r;
    }

    fn calculate_position(&mut self, b_box: Dimensions) {
        let style = self.get_style_node()
    }

    fn calculate_height(&mut self) {
        
    }

    fn layout_children(&mut self) {
        
    }

    fn get_style_node(&self) -> &StyledNode {
        match self.box_type {
            BoxType::Block(n) => n,
            BoxType::Inline(n) => n,
            BoxType::Anonymous => panic!("anonymous block has no associated style node"),
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

fn build_layout_tree<'a>(node: &'a StyledNode) -> LayoutBox<'a> {
    let mut rect = LayoutBox::new(match node.get_display() {
        Display::Block => BoxType::Block(node),
        Display::Inline => BoxType::Inline(node),
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