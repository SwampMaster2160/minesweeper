
use glium::glutin::event;
use glium::{self, uniforms, Blend};
use glium::{glutin, glutin::{event_loop, window, dpi}, Surface};
use std::io::Cursor;

mod vertex;
mod texture;
mod bord;
mod cell;

fn main() {
	let mut bord = bord::Bord::new([10, 12]);
	let bord_size = bord.get_size_in_pixels();
	let header_size = 100;

	// Setup window
	let events_loop = event_loop::EventLoop::new();
	let window_builder = window::WindowBuilder::new()
		.with_inner_size(dpi::LogicalSize::new(bord_size[0] as u32, bord_size[1] as u32 + header_size as u32)).with_title("Minesweeper").with_resizable(false);
	let context_builder = glutin::ContextBuilder::new().with_vsync(true);
	let display = glium::Display::new(window_builder, context_builder, &events_loop).unwrap();
	let mut window_scale = display.get_framebuffer_dimensions().0 as f32 / 640.;

	// Create texture
	let image = image::load(Cursor::new(&include_bytes!("textures.png")),
						image::ImageFormat::Png).unwrap().to_rgba8();
	let image_dimensions = image.dimensions();
	let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
	let texture = glium::texture::SrgbTexture2d::new(&display, image).unwrap();

	// Create program
	let vertex_shader = include_str!("vertex_shader.glsl");
	let fragment_shader = include_str!("fragment_shader.glsl");
	let program = glium::Program::from_source(&display, vertex_shader, fragment_shader, None).unwrap();

	// Behavior
	let behavior = uniforms::SamplerBehavior {
		minify_filter: uniforms::MinifySamplerFilter::Nearest,
		magnify_filter: uniforms::MagnifySamplerFilter::Nearest,
		..Default::default()
	};
	let draw_parameters = glium::DrawParameters {
		blend: Blend::alpha_blending(),
		..glium::DrawParameters::default()
	};

	// Vars
	let mut cursor_pos = [0u16; 2];
	let mut window_size = [640, 480];

	// Game loop
	events_loop.run(move |event, _, control_flow| {
		*control_flow = glutin::event_loop::ControlFlow::Poll;
		match event {
			glutin::event::Event::WindowEvent { event, .. } => match event {
				// On exit button press
				event::WindowEvent::CloseRequested => *control_flow = glutin::event_loop::ControlFlow::Exit,
				// On cursor move
				event::WindowEvent::CursorMoved { device_id: _, position, .. } => cursor_pos = [
						(position.x as f32 / window_scale) as u16, (position.y as f32 / window_scale) as u16
					],
				// Window resize
				event::WindowEvent::Resized(size) => window_size = [size.width as u16, size.height as u16],
				event::WindowEvent::ScaleFactorChanged { scale_factor, .. } => window_scale = scale_factor as f32,
				// Mouse click
				event::WindowEvent::MouseInput { device_id: _, state, button, .. } => {
					
				}
				_ => {}
			},

			// Draw
			glutin::event::Event::MainEventsCleared => {
				// Get frame for drawing on
				let mut frame = display.draw();
				frame.clear_color(0.4, 0.4, 0.4, 0.);

				// Get tris
				let mut gui_tris: Vec<vertex::Vertex> = Vec::new();
				gui_tris.extend(bord.draw(header_size));
				let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

				// Draw tris
				let gui_vertex_buffer = glium::vertex::VertexBuffer::new(&display, &gui_tris).unwrap();
				let gui_uniforms = glium::uniform! {
					matrix: [
						[1. / window_size[0] as f32 * 2. * window_scale, 0., 0., 0.],
						[0., -1. / window_size[1] as f32 * 2. * window_scale, 0., 0.],
						[0., 0., 0., 0.],
						[-1., 1., 0., 1.0f32],
					],
					texture_sampler: uniforms::Sampler(&texture, behavior),
				};
				frame.draw(&gui_vertex_buffer, &indices, &program, &gui_uniforms, &draw_parameters).unwrap();

				frame.finish().unwrap();
			}
			_ => {}
		}
	});
}