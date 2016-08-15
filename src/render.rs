//! The `render` module takes a tree of layout boxes and paints the screen.
// use layout::{LayoutBox, Rect};
// use css::{Color}; 

// type DisplayList = Vec<DisplayCommand>;

// struct DisplayCommand {
//     SolidRect(Color, Rect),
// }

// fn build_display_commands(root: &LayoutBox) -> DisplayList {
//     let mut commands = Vec::new();
//     renter_layout_box(&mut commands, root);
//     commands
// }

// fn render_layout_box(commands: &mut DisplayList, layout_box: &LayoutBox) {
//     render_background(commands, layout_box);
//     render_borders(commands, layout_box);
//     //TODO: Render text

//     for child in &layout_box.children {
//         render_layout_box(commands, child);
//     }
// }