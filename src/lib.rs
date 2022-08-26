pub mod one1fy_platform {
    type OneEventLoop as glutin::event_loop::EventLoop;

    struct OnePlatform {
        Surface surface: skia_safe::Surface,
        event_loop: EventLoop,
    };

    #[cfg(feature = "windows")]
    fn build_platform() -> Option<OnePlatform> {
        use gl::types::*;
        use glutin::{
            event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent},
            event_loop::{ControlFlow},
            window::WindowBuilder,
            GlProfile,
        };
        use skia_safe::{
            gpu::{
                gl::FramebufferInfo, BackendRenderTarget, SurfaceOrigin
            },
            Color,
            ColorType,
            Surface,
        };

        const WIDTH: f64 = 375.0;
        const HEIGHT: f64 = 667.0;

        type WindowedContext = glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>;

        let el = EventLoop::new();
        let wb = WindowBuilder::new().with_title("One1fy Windows Viewer");

        let cb = glutin::ContextBuilder::new()
            .with_depth_buffer(0)
            .with_stencil_buffer(8)
            .with_pixel_format(24, 8)
            .with_gl_profile(GlProfile::Core);

        let cb = cb.with_double_buffer(Some(true));

        let windowed_context = cb.build_windowed(wb, &el).unwrap();

        let windowed_context = unsafe { windowed_context.make_current().unwrap() };
        let pixel_format = windowed_context.get_pixel_format();

        println!(
            "Pixel format of the window's GL context: {:?}",
            pixel_format
        );

        gl::load_with(|s| windowed_context.get_proc_address(s));

        let mut gr_context = skia_safe::gpu::DirectContext::new_gl(None, None).unwrap();

        let fb_info = {
            let mut fboid: GLint = 0;
            unsafe { gl::GetIntegerv(gl::FRAMEBUFFER_BINDING, &mut fboid) };

            FramebufferInfo {
                fboid: fboid.try_into().unwrap(),
                format: skia_safe::gpu::gl::Format::RGBA8.into(),
            }
        };

        windowed_context
            .window()
            .set_inner_size(
                glutin::dpi::Size::new(
                    glutin::dpi::LogicalSize::new(
                        WIDTH,
                        HEIGHT,
                    )
                )
            );

        fn create_surface(
            windowed_context: &WindowedContext,
            fb_info: &FramebufferInfo,
            gr_context: &mut skia_safe::gpu::DirectContext,
        ) -> skia_safe::Surface {
            let pixel_format = windowed_context.get_pixel_format();
            let size = windowed_context.window().inner_size();

            let backend_render_target = BackendRenderTarget::new_gl(
                (
                    size.width.try_into().unwrap(),
                    size.height.try_into().unwrap(),
                ),
                pixel_format.multisampling.map(|s| s.try_into().unwrap()),
                pixel_format.stencil_bits.try_into().unwrap(),
                *fb_info,
            );

            Surface::from_backend_render_target(
                gr_context,
                &backend_render_target,
                SurfaceOrigin::BottomLeft,
                ColorType::RGBA8888,
                None,
                None,
            )
            .unwrap()
        }

        let surface = create_surface(&windowed_context, &fb_info, &mut gr_context);

        let mut frame = 0;

        struct Env {
            surface: Surface,
            gr_context: skia_safe::gpu::DirectContext,
            windowed_context: WindowedContext,
        }

        let mut env = Env {
            surface,
            gr_context,
            windowed_context,
        };

        el.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            #[allow(deprecated)]
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
                                virtual_keycode,
                                modifiers,
                                ..
                            },
                        ..
                    } => {
                        if modifiers.logo() {
                            if let Some(VirtualKeyCode::Q) = virtual_keycode {
                                *control_flow = ControlFlow::Exit;
                            }
                        }
                        frame += 1;
                        env.windowed_context.window().request_redraw();
                    }
                    _ => (),
                },
                Event::RedrawRequested(_) => {
                    {
                        let canvas = env.surface.canvas();
                        canvas.clear(Color::WHITE);
                        // renderer::render_frame(frame % 360, 12, 60, canvas);
                    }
                    env.surface.canvas().flush();
                    env.windowed_context.swap_buffers().unwrap();
                }
                _ => (),
            }
        });
    }

    #[cfg(not(feature = "windows"))]
    fn build_platform() -> Option<OnePlatform> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
