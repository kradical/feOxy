//! The `render` module takes a tree of layout boxes and paints the screen.

// TODO move this file into it's own module
// TODO seperate drawing logic from command building logic

use gfx;
use gfx_text;
use gfx_window_glutin;
use glutin;

use gfx::Factory;
use gfx::traits::FactoryExt;
use gfx::Device;

use layout;
use command::DisplayCommand;

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

#[derive(Copy, Clone)]
struct RenderText<'a> {
    text: &'a str,
    position: [i32; 2],
    color: [f32; 4],
}

const CLEAR_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

pub fn render_loop(command_list: &[DisplayCommand]) {
    let builder = glutin::WindowBuilder::new()
        .with_title(String::from("feOxy"))
        .with_dimensions(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
        .with_vsync();

    let (window, mut device, mut factory, main_color, _main_depth) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder);

    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    let pso = factory.create_pipeline_simple(
        include_bytes!("shaders/solid.glslv"),
        include_bytes!("shaders/solid.glslf"),
        pipe::new()
    ).unwrap();

    let (vertices, index_data) = render_commands(command_list);
    let texts = render_texts(command_list);

    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&vertices, &index_data[..]);

    let data = pipe::Data {
        vbuf: vertex_buffer,
        out: main_color,
    };

    // Initialize text renderer.
    let mut textRenderer = gfx_text::new(factory).build().unwrap();

    'main: loop {
        for event in window.poll_events() {
            match event {
                glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape)) |
                glutin::Event::Closed => break 'main,
                _ => {},
            }
        }

        for text in &texts {
            textRenderer.add(
                text.text,
                text.position,
                text.color,
            );
        }

        encoder.clear(&data.out, CLEAR_COLOR);
        
        encoder.draw(&slice, &pso, &data);
        textRenderer.draw(&mut encoder, &data.out);
        
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
    }
}

fn render_texts(command_list: &[DisplayCommand]) -> Vec<RenderText> {
    Vec::new()
}

fn render_commands(command_list: &[DisplayCommand]) -> (Vec<Vertex>, Vec<u16>) {
    let mut vertices = Vec::new();
    let mut index_data = Vec::new();
    let mut rect_num: u16 = 0;

    for command in command_list {
        match *command {
            DisplayCommand::SolidRect(ref color, ref rect) => {
                let c = [color.r, color.g, color.b];

                let mut v = render_rect(&c, rect);
                vertices.append(&mut v);

                let index_base: u16 = rect_num * 4;
                index_data.append(&mut vec![index_base, index_base + 1, index_base + 2, index_base + 2, index_base + 3, index_base]);
                rect_num += 1;
            },
        }
    }
    return (vertices, index_data);
}

fn render_rect(c: &[f32; 3], rect: &layout::Rect) -> Vec<Vertex> {
    let (x, y, h, w) = transform_rect(rect);
    let vertices = vec![
        Vertex { pos: [x + w, y], color: *c },
        Vertex { pos: [x, y], color: *c },
        Vertex { pos: [x, y + h], color: *c },
        Vertex { pos: [x + w, y + h], color: *c },
    ];

    vertices
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
fn transform_rect(rect: &layout::Rect) -> (f32, f32, f32, f32) {
    let w = rect.width / SCREEN_WIDTH as f32 * 2.0;
    let h = rect.height / SCREEN_HEIGHT as f32 * 2.0;
    let x = rect.x / SCREEN_WIDTH as f32 * 2.0 - 1.0;
    let y = -(rect.y / SCREEN_HEIGHT as f32 * 2.0 - 1.0 + h);

    (x, y, h, w)
}
