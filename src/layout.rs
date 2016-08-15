use style::{StyledNode, Display};
use std::fmt;

pub struct LayoutBox<'a> {
    dimensions: Dimensions,
    box_type: BoxType<'a>,
    pub children: Vec<LayoutBox<'a>>,
}

pub enum BoxType<'a> {
    Block(&'a StyledNode<'a>),
    Inline(&'a StyledNode<'a>),
    Anonymous,
}

#[derive(Clone, Copy, Default)]
pub struct Dimensions {
    pub content: Rect,
    padding: EdgeSizes,
    border: EdgeSizes,
    margin: EdgeSizes,
}

#[derive(Clone, Copy, Default)]
pub struct Rect {
    x: f32,
    y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Clone, Copy, Default)]
struct EdgeSizes {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
}

impl<'a> LayoutBox<'a> {
    /// Constructs a new LayoutBox
    ///
    /// box_type: the type of layout box to create.
    pub fn new(box_type: BoxType) -> LayoutBox {
        LayoutBox {
            box_type: box_type,
            dimensions: Default::default(),
            children: Vec::new(),
        }
    }

    /// Returns either the current inline/anonymous box or creates a new one
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

    /// Lays out the current box, including recursively laying out children boxes.
    ///
    /// b_box: the parent bounding box.
    fn layout(&mut self, b_box: Dimensions) {
        match self.box_type {
            BoxType::Block(_) => self.layout_block(b_box),
            BoxType::Inline(_) => self.layout_block(b_box),
            BoxType::Anonymous => {},
        }
    }

    /// Calls all the functions to layout the current layout box.
    ///
    /// b_box: the parent bounding box.
    fn layout_block(&mut self, b_box: Dimensions)  {
        self.calculate_width(b_box);
        self.calculate_position(b_box);
        self.layout_children();
        self.calculate_height();
    }

    /// Update the current layout box's width dimensions.
    ///
    /// b_box: the parent bounding box.
    fn calculate_width(&mut self, b_box: Dimensions) {
        let style = self.get_style_node();
        let d = &mut self.dimensions;
        
        let mut width: f32 = style.value_or("width", 0.0);
        let mut margin_l: f32 = style.value_or("margin-left", 0.0);
        let mut margin_r: f32 = style.value_or("margin-right", 0.0);
        
        d.border.left = style.value_or("border-left-width", 0.0);
        d.border.right = style.value_or("border-right-width", 0.0);
        d.padding.left = style.value_or("padding-left", 0.0);
        d.padding.right = style.value_or("padding-right", 0.0);

        let total = width + margin_l + margin_r + d.border.left + d.border.right + d.padding.left
            + d.padding.right;

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

        d.content.width = width;
        d.margin.left =  margin_l;
        d.margin.right = margin_r;
    }

    /// Position current box below previous boxes in container by updating height
    fn calculate_position(&mut self, b_box: Dimensions) {
        let style = self.get_style_node();
        let d = &mut self.dimensions;

        d.margin.top = style.value_or("margin-top", 0.0);
        d.margin.bottom = style.value_or("margin-top", 0.0);
        d.border.top = style.value_or("border-top-width", 0.0);
        d.border.bottom = style.value_or("border-top-width", 0.0);
        d.padding.top = style.value_or("padding-top", 0.0);
        d.padding.bottom = style.value_or("padding-top", 0.0);

        d.content.x = b_box.content.x + d.margin.left + d.border.left + d.padding.left;
        d.content.y = b_box.content.height + b_box.content.y + d.margin.top + d.border.top
            + d.padding.top;
    }

    /// Use a style node's height value if it exists
    fn calculate_height(&mut self) {
        match self.get_style_node().value("height") {
            Some(h) => match h.parse::<f32>() {
                Ok(num) => { self.dimensions.content.height = num; },
                Err(_) => {}
            },
            None => {}
        }
    }

    /// Layout the current nodes children and adjust it's height.
    fn layout_children(&mut self) {
        let d = &mut self.dimensions;

        for child in &mut self.children {
            child.layout(*d);
            d.content.height += child.dimensions.margin_box().height;
        }
    }

    /// Return the style node for the layout block.
    fn get_style_node(&self) -> &'a StyledNode<'a> {
        match self.box_type {
            BoxType::Block(n) => n,
            BoxType::Inline(n) => n,
            BoxType::Anonymous => panic!("anonymous blocks have no associated style node"),
        }
    }
}
impl<'a> fmt::Debug for LayoutBox<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "type:\n  {:?}\n{:?}\n", self.box_type, self.dimensions)
    } 
}

impl Dimensions {
    /// Updates content size to include paddings.
    fn padding_box(&self) -> Rect {
        self.content.expanded(self.padding)
    }

    /// Updates content size to include borders.
    fn border_box(&self) -> Rect {
        self.padding_box().expanded(self.border)
    }

    /// Updates content size to include margins.
    fn margin_box(&self) -> Rect {
        self.border_box().expanded(self.margin)
    }

}
impl fmt::Debug for Dimensions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "content:\n  {:?}\npadding:\n  {:?}\nborder:\n  {:?}\nmargin:\n  {:?}",
            self.content, self.padding, self.border, self.margin)
    }
}

impl Rect {
    /// Expands a rect with the given dimensions.
    ///
    /// e: the EdgeSizes to expand by.
    /// TODO margin collapsing
    fn expanded(&self, e: EdgeSizes) -> Rect {
        Rect {
            x: self.x - e.left,
            y: self.y - e.top,
            width: self.width + e.left + e.right,
            height: self.height + e.top + e.bottom,
        }
    }
}
impl fmt::Debug for Rect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}, w: {}, h: {}", self.x, self.y, self.width, self.height)
    }
}

impl fmt::Debug for EdgeSizes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "l: {} r: {} top: {} bot: {}", self.left, self.right, self.top, self.bottom)
    }
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

/// Entry point to create a layout tree.
///
/// root: The root of the style tree to layout. 
/// containing_block: The window or viewport.
pub fn layout_tree<'a>(root: &'a StyledNode<'a>, mut containing_block: Dimensions) -> LayoutBox<'a> {
    // The layout algorithm expects the container height to start at 0.
    // TODO: Save the initial containing block height, for calculating percent heights.
    containing_block.content.height = 0.0;

    let mut root_box = build_layout_tree(root);
    root_box.layout(containing_block);
    return root_box;
}

/// Recursively builds the layout tree.
///
/// node: The current style node being laid out.
fn build_layout_tree<'a>(node: &'a StyledNode) -> LayoutBox<'a> {
    let mut layout_node = LayoutBox::new(match node.get_display() {
        Display::Block => BoxType::Block(node),
        Display::Inline => BoxType::Inline(node),
        Display::None => panic!("root node has display: none")
    });

    for child in &node.children {
        match child.get_display() {
            Display::Block => layout_node.children.push(build_layout_tree(child)),
            Display::Inline => layout_node.get_inline().children.push(build_layout_tree(child)),
            Display::None => {}
        }
    }
    layout_node
}

/// Print a layout node and it's descendents
///
/// n: The node of the style tree to print.
pub fn pretty_print<'a>(n: &'a LayoutBox) {
    println!("{:?}\n", n);

    for child in n.children.iter() {
        pretty_print(&child);
    }
}

/// Tests ----------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    /// Test
    #[test]
    fn it_works() {

    }
}