//! The `render` module takes a tree of layout boxes and paints the screen.

// TODO move this file into it's own module
// TODO seperate drawing logic from command building logic

use gfx;
use gfx_window_glutin;
use glutin;

use gfx::traits::FactoryExt;
use gfx::Device;

type DepthFormat = gfx::format::DepthStencil;

gfx_defines!{
    vertex Vertex {
        pos: [f32; 2] = "a_Pos",
        color: [f32; 3] = "a_Color",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        out: gfx::RenderTarget<gfx::format::Rgba8> = "Target0",
    }
}

const TRIANGLE: [Vertex; 3] = [
    Vertex { pos: [-0.5, -0.5], color: [1.0, 0.0, 0.0] },
    Vertex { pos: [0.5, -0.5], color: [0.0, 1.0, 0.0] },
    Vertex { pos: [0.0, 0.5], color: [0.0, 0.0, 1.0] },
];

const CLEAR_COLOR: [f32; 4] = [0.1, 0.2, 0.3, 1.0];

pub fn render_triangle() {
    let builder = glutin::WindowBuilder::new()
        .with_title(String::from("TRIANGLE"))
        .with_dimensions(1024, 768)
        .with_vsync();

    let (window, mut device, mut factory, main_color, _main_depth) =
        gfx_window_glutin::init::<gfx::format::Rgba8, DepthFormat>(builder);

    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    let pso = factory.create_pipeline_simple(
        include_bytes!("shaders/triangle.glslv"),
        include_bytes!("shaders/triangle.glslf"),
        pipe::new()
    ).unwrap();

    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&TRIANGLE, ());

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
