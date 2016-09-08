//! The `render` module takes a tree of layout boxes and paints the screen.
use layout::{LayoutBox, Rect};

pub type DisplayList = Vec<DisplayCommand>;

pub enum DisplayCommand {
    SolidRect(String, Rect),
}

pub fn build_display_commands(root: &LayoutBox) -> DisplayList {
    let mut commands = Vec::new();
    render_layout_box(&mut commands, root);
    commands
}

fn render_layout_box(commands: &mut DisplayList, layout_box: &LayoutBox) {
    render_background(commands, layout_box);
    render_borders(commands, layout_box);
    //TODO: Render text

    for child in &layout_box.children {
        render_layout_box(commands, child);
    }
}

fn render_background(commands: &mut DisplayList, layout_box: &LayoutBox) {
    get_color(layout_box, "background").map(|color|
        commands.push(DisplayCommand::SolidRect(color, layout_box.dimensions.border_box())));
}

fn get_color(layout_box: &LayoutBox, name: &str) -> Option<String> {
    return None;
}

fn render_borders(commands: &mut DisplayList, layout_box: &LayoutBox) {
    let color = match get_color(layout_box, "border-color") {
        Some(color) => color,
        _ => return,
    };

    let d = &layout_box.dimensions;
    let border_box = d.border_box();

    commands.push(DisplayCommand::SolidRect(color.clone(), Rect {
        x: border_box.x,
        y: border_box.y,
        width: d.border.left,
        height: border_box.height,
    }));

    commands.push(DisplayCommand::SolidRect(color.clone(), Rect {
        x: border_box.x + border_box.width - d.border.right,
        y: border_box.y,
        width: d.border.right,
        height: border_box.height,
    }));

    commands.push(DisplayCommand::SolidRect(color.clone(), Rect {
        x: border_box.x,
        y: border_box.y,
        width: border_box.width,
        height: d.border.top,
    }));

    commands.push(DisplayCommand::SolidRect(color.clone(), Rect {
        x: border_box.x,
        y: border_box.y + border_box.height - d.border.bottom,
        width: border_box.width,
        height: d.border.bottom,
    }));
}
