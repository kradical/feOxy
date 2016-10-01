//! The `layout` module takes a style tree and creates a layout of boxes.

use css::Value;
use style::{StyledNode, Display};
use std::fmt;

pub struct LayoutBox<'a> {
    pub dimensions: Dimensions,
    box_type: BoxType<'a>,
    pub children: Vec<LayoutBox<'a>>,
}

pub enum BoxType<'a> {
    Block(&'a StyledNode<'a>),
    Inline(&'a StyledNode<'a>),
    InlineBlock(&'a StyledNode<'a>),
    Anonymous,
}

#[derive(Clone, Copy, Default)]
pub struct Dimensions {
    pub content: Rect,
    padding: EdgeSizes,
    pub border: EdgeSizes,
    margin: EdgeSizes,
    current: Rect,
}

#[derive(Clone, Copy, Default)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Clone, Copy, Default)]
pub struct EdgeSizes {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
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

    /// Lays out the current box, including recursively laying out children boxes.
    ///
    /// b_box: the parent bounding box.
    fn layout(&mut self, b_box: Dimensions) {
        match self.box_type {
            BoxType::Block(_) => self.layout_block(b_box),
            BoxType::Inline(_) => self.layout_block(b_box), // TODO
            BoxType::InlineBlock(_) => self.layout_inline_block(b_box),
            BoxType::Anonymous => {}, // TODO
        }
    }

    fn layout_inline_block(&mut self, b_box: Dimensions) {
        self.calculate_inline_width(); // width in pixels for now
        self.calculate_inline_position(b_box);
        self.layout_children();
        self.calculate_height();
    }

    fn calculate_inline_width(&mut self) {
        let s = self.get_style_node();
        let d = &mut self.dimensions;

        d.content.width = s.num_or("width", 0.0);
        d.margin.left = s.num_or("margin-left", 0.0);
        d.margin.right = s.num_or("margin-right", 0.0);
        d.padding.left = s.num_or("padding-left", 0.0);
        d.padding.right = s.num_or("padding-right", 0.0);
        d.border.left = s.num_or("border-left-width", 0.0);
        d.border.right = s.num_or("border-right-width", 0.0);
    }

    /// Position current box below previous boxes in container by updating height
    fn calculate_inline_position(&mut self, b_box: Dimensions) {
        let style = self.get_style_node();
        let d = &mut self.dimensions;

        d.margin.top = style.num_or("margin-top", 0.0);
        d.margin.bottom = style.num_or("margin-bottom", 0.0);
        d.border.top = style.num_or("border-top-width", 0.0);
        d.border.bottom = style.num_or("border-bottom-width", 0.0);
        d.padding.top = style.num_or("padding-top", 0.0);
        d.padding.bottom = style.num_or("padding-bottom", 0.0);

        d.content.x = b_box.content.x + b_box.current.x + d.margin.left + d.border.left + d.padding.left;
        d.content.y = b_box.content.height + b_box.content.y + d.margin.top + d.border.top
            + d.padding.top;
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

        let width = style.num_or("width", 0.0);
        let margin_l = style.value("margin-left");
        let margin_r = style.value("margin-right");

        let margin_l_num = match margin_l {
            Some(m) => match **m {
                Value::Other(ref s) => s.parse().unwrap_or(0.0),
                _ => 0.0,
            },
            None => 0.0,
        };
        let margin_r_num = match margin_r {
            Some(m) => match **m {
                Value::Other(ref s) => s.parse().unwrap_or(0.0),
                _ => 0.0,
            },
            None => 0.0,
        };

        d.border.left = style.num_or("border-left-width", 0.0);
        d.border.right = style.num_or("border-right-width", 0.0);
        d.padding.left = style.num_or("padding-left", 0.0);
        d.padding.right = style.num_or("padding-right", 0.0);

        let total = width + margin_l_num + margin_r_num + d.border.left + d.border.right + d.padding.left
            + d.padding.right;

        let underflow = b_box.content.width - total;

        match (width, margin_l, margin_r) {
            // width is auto
            (0.0, _, _) => {
                if underflow >= 0.0 {
                    d.content.width = underflow;
                    d.margin.right = margin_r_num;
                } else {
                    // width can't be negative
                    d.margin.right = margin_r_num + underflow;
                    d.content.width = width;
                }
                d.margin.left = margin_l_num;
            },
            // left margin is auto
            (w, None, Some(_)) if w != 0.0 => {
                d.margin.left = underflow;
                d.margin.right = margin_r_num;
                d.content.width = w;
            },
            // right margin is auto
            (w, Some(_), None) if w != 0.0 => {
                d.margin.right = underflow;
                d.margin.left = margin_l_num;
                d.content.width = w;
            },
            // left/right margin are auto
            (w, None, None) if w != 0.0 => {
                d.margin.left = underflow / 2.0;
                d.margin.right = underflow / 2.0;
                d.content.width = w;
            },
            // values are overconstrained, calculate margin_right.
            (_, _, _) => {
                d.margin.right = margin_r_num + underflow;
                d.margin.left = margin_l_num;
                d.content.width = width
            },
        }
    }

    /// Position current box below previous boxes in container by updating height
    fn calculate_position(&mut self, b_box: Dimensions) {
        let style = self.get_style_node();
        let d = &mut self.dimensions;

        d.margin.top = style.num_or("margin-top", 0.0);
        d.margin.bottom = style.num_or("margin-bottom", 0.0);
        d.border.top = style.num_or("border-top-width", 0.0);
        d.border.bottom = style.num_or("border-bottom-width", 0.0);
        d.padding.top = style.num_or("padding-top", 0.0);
        d.padding.bottom = style.num_or("padding-bottom", 0.0);

        d.content.x = b_box.content.x + d.margin.left + d.border.left + d.padding.left;
        d.content.y = b_box.content.height + b_box.content.y + d.margin.top + d.border.top
            + d.padding.top;
    }

    /// Use a style node's height value if it exists
    fn calculate_height(&mut self) {
        self.get_style_node().value("height").map_or((), |h| {
            match **h {
                Value::Length(n, _) => self.dimensions.content.height = n,
                _ => {},
            }
        })
    }

    /// Layout the current nodes children and adjust it's height.
    fn layout_children(&mut self) {
        let d = &mut self.dimensions;
        let mut max_child_height = 0.0;

        for child in &mut self.children {
            child.layout(*d);
            let new_height = child.dimensions.margin_box().height;

            if new_height > max_child_height {
                max_child_height = new_height;
            }

            match child.box_type {
                BoxType::Block(_) => d.content.height += child.dimensions.margin_box().height,
                BoxType::InlineBlock(_) => {
                    d.current.x += child.dimensions.margin_box().width;

                    if d.current.x > d.content.width {
                        d.content.height += max_child_height;
                        d.current.x = 0.0;
                        child.layout(*d); // relayout child
                        d.current.x += child.dimensions.margin_box().width;
                    }
                },
                _ => {},
            }
        }
    }

    /// Return the style node for the layout block.
    pub fn get_style_node(&self) -> &'a StyledNode<'a> {
        match self.box_type {
            BoxType::Block(n) => n,
            BoxType::Inline(n) => n,
            BoxType::InlineBlock(n) => n,
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
    pub fn border_box(&self) -> Rect {
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
            BoxType::InlineBlock(_) => "inline-block",
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
        Display::InlineBlock => BoxType::InlineBlock(node),
        Display::None => panic!("root node has display: none")
    });

    for child in &node.children {
        match child.get_display() {
            Display::Block => layout_node.children.push(build_layout_tree(child)),
            Display::Inline => layout_node.children.push(build_layout_tree(child)),
            Display::InlineBlock => layout_node.children.push(build_layout_tree(child)),
            Display::None => {}
        }
    }
    layout_node
}

/// Print a layout node and it's descendents
///
/// n: The node of the style tree to print.
pub fn pretty_print<'a>(n: &'a LayoutBox, level: usize) {
    println!("{}{:?}\n", level, n);

    for child in n.children.iter() {
        pretty_print(&child, level + 1);
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
