use crate::orchestrator::redraw::handle_redraw;
use crate::orchestrator::event::click::handle_click;
use crate::orchestrator::event::keypress::handle_press;
use crate::components::BarContainer;
use crate::components::ComponentTraits;

#[cfg(feature = "macos")]
use skia_safe::{scalar, ColorType, Size, Surface};

#[cfg(feature = "macos")]
pub fn start_event_loop(mut tree: BarContainer) {
    use cocoa::{appkit::NSView, base::id as cocoa_id};

    use core_graphics_types::geometry::CGSize;

    use foreign_types_shared::{ForeignType, ForeignTypeRef};
    use metal_rs::{Device, MTLPixelFormat, MetalLayer};
    use objc::{runtime::YES};

    use skia_safe::gpu::{mtl, BackendRenderTarget, DirectContext, SurfaceOrigin};

    use winit::{
        dpi::{LogicalSize, PhysicalPosition},
        event::{Event, WindowEvent, KeyboardInput, ElementState},
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

    let mut last_position = PhysicalPosition::<f64>::new(0.0, 0.0);

    events_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::LoopDestroyed => {}
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(size) => {
                    metal_layer
                        .set_drawable_size(CGSize::new(size.width as f64, size.height as f64));
                    window.request_redraw()
                }
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput {
                    input,
                    ..
                } => {
                    if input.state == ElementState::Pressed {
                        handle_press(input, &mut tree);
                    }
                    
                }
                WindowEvent::CursorMoved {
                    position,
                    ..
                } => {
                    last_position = position;
                }
                WindowEvent::MouseInput {
                    state,
                    button,
                    modifiers: _,
                    ..
                } => {
                    if state == ElementState::Pressed {
                        handle_click(last_position, state, button, &mut tree)
                    }
                    
                }
                _ => (),
            },
            Event::RedrawRequested(_) => {}
            _ => (),
        }
        if let Some(drawable) = metal_layer.next_drawable() {
            let drawable_size = {
                let size = metal_layer.drawable_size();
                Size::new(size.width as scalar, size.height as scalar)
            };
            let mut surface = unsafe {
                let texture_info = 
                    mtl::TextureInfo::new(drawable.texture().as_ptr() as mtl::Handle);
                let backend_render_target = BackendRenderTarget::new_metal(
                    (drawable_size.width as i32, drawable_size.height as i32),
                    1,
                    &texture_info,
                );
                Surface::from_backend_render_target(
                    &mut context,
                    &backend_render_target,
                    SurfaceOrigin::TopLeft,
                    ColorType::BGRA8888,
                    None,
                    None,
                )
                .unwrap()
    
            };
            handle_redraw(surface.canvas(), &mut tree);
            surface.flush_and_submit();
            drop(surface); 
            let command_buffer = command_queue.new_command_buffer();
            command_buffer.present_drawable(drawable);
            command_buffer.commit();
        }
        
    });
}