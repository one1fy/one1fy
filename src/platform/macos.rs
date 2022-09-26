use crate::orchestrator::redraw::handle_redraw;
use crate::orchestrator::event::click::handle_click;
use crate::components::BoxComponent;
use rand::Rng;

// #[cfg(not(target_os = "macos"))]
// fn main() {
//     println!("This example is only supported on macos")
// }

// #[cfg(all(target_os = "macos", not(feature = "metal")))]
// fn main() {
//     println!("To run this example, invoke cargo with --features \"metal\".")
// }

#[cfg(all(target_os = "macos", feature = "metal"))]
use skia_safe::{scalar, Canvas, Color4f, ColorType, Paint, Point, Rect, Size, Surface};

#[cfg(all(target_os = "macos", feature = "metal"))]
pub fn start_event_loop_mac(mut tree: BoxComponent) {
    use cocoa::{appkit::NSView, base::id as cocoa_id};

    use core_graphics_types::geometry::CGSize;

    use foreign_types_shared::{ForeignType, ForeignTypeRef};
    use metal_rs::{Device, MTLPixelFormat, MetalLayer};
    use objc::{rc::autoreleasepool, runtime::YES};

    use skia_safe::gpu::{mtl, BackendRenderTarget, DirectContext, SurfaceOrigin};

    use winit::{
        dpi::LogicalSize,
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        platform::macos::WindowExtMacOS,
        window::WindowBuilder,
    };

    const WIDTH: f64 = 375.0;
    const HEIGHT: f64 = 667.0;

    let size = LogicalSize::new(WIDTH, HEIGHT);

    let events_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_inner_size(size)
        .with_title("One1fy MacOS example".to_string())
        .build(&events_loop)
        .unwrap();

    let device = Device::system_default().expect("no device found");

    let metal_layer = {
        let draw_size = window.inner_size();
        let layer = MetalLayer::new();
        layer.set_device(&device);
        layer.set_pixel_format(MTLPixelFormat::BGRA8Unorm);
        layer.set_presents_with_transaction(false);

        unsafe {
            let view = window.ns_view() as cocoa_id;
            view.setWantsLayer(YES);
            view.setLayer(layer.as_ref() as *const _ as _);
        }
        layer.set_drawable_size(CGSize::new(draw_size.width as f64, draw_size.height as f64));
        layer
    };

    let command_queue = device.new_command_queue();

    let backend = unsafe {
        mtl::BackendContext::new(
            device.as_ptr() as mtl::Handle,
            command_queue.as_ptr() as mtl::Handle,
            std::ptr::null(),
        )
    };

    let mut context = DirectContext::new_metal(&backend, None).unwrap();

    events_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::LoopDestroyed => {}
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical_size) => {
                    env.surface = create_surface(
                        &env.windowed_context,
                        &fb_info,
                        &mut env.gr_context
                    );
                    env.windowed_context.resize(physical_size)
                }
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: _,
                            modifiers: _,
                            ..
                        },
                    ..
                } => {

                }
                WindowEvent::CursorMoved {
                    position,
                    ..
                } => {
                    last_postition = position;
                }
                WindowEvent::MouseInput {
                    state,
                    button,
                    modifiers: _,
                    ..
                } => {
                    handle_click(last_postition, state, button)
                }
                _ => (),
            },
            Event::RedrawRequested(_) => {}
            _ => (),
        }
        draw(surface.canvas())
        surface.flush_and_submit()
        surface.drop();        
    };
}

/// Renders a rectangle that occupies exactly half of the canvas
#[cfg(all(target_os = "macos", feature = "metal"))]
fn draw(canvas: &mut Canvas) {
    let mut rng = rand::thread_rng();
    let col: f64 = rng.gen_range(0.0..1.0);
    let canvas_size = Size::from(canvas.base_layer_size());

    canvas.clear(Color4f::new(1.0, 1.0, 1.0, 1.0));

    let rect_size = canvas_size / 2.0;
    let rect = Rect::from_point_and_size(
        Point::new(
            (canvas_size.width - rect_size.width) / 2.0,
            (canvas_size.height - rect_size.height) / 2.0,
        ),
        rect_size,
    );
    canvas.draw_rect(rect, &Paint::new(Color4f::new(0.0, 0.0, col, col), None));
}