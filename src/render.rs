//! The `render` module takes a tree of layout boxes and paints the screen.

// TODO move this file into it's own module
// TODO seperate drawing logic from command building logic

use gfx;
use gfx_window_glutin;
use glutin;

use gfx::traits::FactoryExt;
use gfx::Device;

pub type DepthFormat = gfx::format::DepthStencil;
pub type ColorFormat = gfx::format::Rgba8;

// TODO integrate with layout module for screen resizing
const SCREEN_WIDTH: usize = 1024;
const SCREEN_HEIGHT: usize = 768;

gfx_defines!{
    vertex Vertex {
        pos: [f32; 2] = "a_Pos",
        color: [f32; 3] = "a_Color",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }
}

const CLEAR_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

pub fn render_loop() {
    let builder = glutin::WindowBuilder::new()
        .with_title(String::from("feOxy"))
        .with_dimensions(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
        .with_vsync();

    let (window, mut device, mut factory, main_color, _main_depth) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder);

    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    let pso = factory.create_pipeline_simple(
        include_bytes!("shaders/triangle.glslv"),
        include_bytes!("shaders/triangle.glslf"),
        pipe::new()
    ).unwrap();

    let (vertices, index_data) = render_commands(vec![
        DisplayCommand::SolidRect(String::from("red"), Rect {x: 100.0, y: 100.0, height: 100.0, width: 100.0 }),
        DisplayCommand::SolidRect(String::from("green"), Rect {x: 0.0, y: 100.0, height: 100.0, width: 100.0 }),
        DisplayCommand::SolidRect(String::from("blue"), Rect {x: 100.0, y: 0.0, height: 100.0, width: 100.0 })
    ]);

    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&vertices, &index_data[..]);

    let data = pipe::Data {
        vbuf: vertex_buffer,
        out: main_color,
    };

    'main: loop {
        for event in window.poll_events() {
            match event {
                glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape)) |
                glutin::Event::Closed => break 'main,
                _ => {},
            }
        }

        encoder.clear(&data.out, CLEAR_COLOR);
        encoder.draw(&slice, &pso, &data);
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
    }
}

fn render_commands(command_list: DisplayList) -> (Vec<Vertex>, Vec<u16>) {
    let mut vertices = Vec::new();
    let mut index_data = Vec::new();

    for command in command_list {
        match command {
            DisplayCommand::SolidRect(color, rect) => {
                let c;
                if color == "red" {
                    c = [0.5, 0.0, 0.0];
                } else if color == "green" {
                    c = [0.0, 0.5, 0.0];
                } else {
                    c = [0.0, 0.0, 0.5];
                };
                let (mut v, mut i) = render_rect(&c, rect);
                vertices.append(&mut v);
                index_data.append(&mut i);
            },
        }
    }
    return (vertices, index_data);
}

fn render_rect(c: &[f32; 3], rect: Rect) -> (Vec<Vertex>, Vec<u16>) {
    println!("{:?}", rect);
    let (x, y, h, w) = transform_rect(rect);
    let vertices = vec![
        Vertex { pos: [x + w, y], color: *c },
        Vertex { pos: [x, y], color: *c },
        Vertex { pos: [x, y + h], color: *c },
        Vertex { pos: [x + w, y + h], color: *c },
    ];
    let index_data = vec![
        0, 1, 2, 2, 3, 0,
        4, 5, 6, 6, 7, 4,
        8, 9, 10, 10, 11, 8,
    ];

    return (vertices, index_data);
}

/// Transforms a rect into gfx coordinates based on screen size.
///
/// layout_box coord system:    gfx obj coord system:  gfx screen coord system:
///  (x, y)    (x+w, y)           (x, y+h) (x+w, y+h)     (-1, 1)    (1,1)
///     +-------+                     +-------+             +-------+
///     |       |                     |       |             |       |
///     |       |                     |       |             |       |
///     +-------+                     +-------+             +-------+
///  (x, y+h) (x+w, y+h)           (x, y)  (x+w, y)     (-1, -1)  (1, -1)
fn transform_rect(rect: Rect) -> (f32, f32, f32, f32) {
    let w = rect.width / SCREEN_WIDTH as f32 * 2.0;
    let h = rect.height / SCREEN_HEIGHT as f32 * 2.0;
    let x = rect.x / SCREEN_WIDTH as f32 * 2.0 - 1.0;
    let y = -(rect.y / SCREEN_HEIGHT as f32 * 2.0 - 1.0 + h);

    println!("x: {}, y: {}, w: {}, h: {}", x, y, w, h);

    (x, y, h, w)
}

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
