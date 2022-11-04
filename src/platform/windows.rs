use crate::orchestrator::redraw::handle_redraw;
use crate::orchestrator::event::click::handle_click;
use crate::components::ComponentTraits;
use crate::components::BarContainer;

#[cfg(feature = "windows")]
pub fn start_event_loop(mut tree: BarContainer) {
    use gl::types::*;
    use glutin::{
        event::{Event, KeyboardInput, WindowEvent, ElementState},
        event_loop::{ControlFlow},
        window::WindowBuilder,
        dpi::PhysicalPosition,
        GlProfile,
    };
    use skia_safe::{
        gpu::{
            gl::FramebufferInfo, BackendRenderTarget, SurfaceOrigin
        },
        ColorType,
        Surface,
    };

    const WIDTH: f64 = 375.0;
    const HEIGHT: f64 = 667.0;

    type WindowedContext = glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>;

    let el = glutin::event_loop::EventLoop::new();
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

    let mut last_postition = PhysicalPosition::<f64>::new(0.0, 0.0);

    #[tokio::main]
    async fn send_request() -> Result<(), reqwest::Error> {
        let res = reqwest::get("https://purdue.edu").await?;

        println!("Status: {}", res.status());

        let url = res.url();

        println!("URL: {}", url);

        open::that("https://purdue.edu").unwrap();

        Ok(())
    }

    use std::fs::File;
    use std::io::BufReader;
    use std::time::Duration;
    use rodio::{Decoder, OutputStream, Sink};
    use rodio::source::{SineWave, Source};

    fn play_sound() {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        // Add a dummy source of the sake of the example.
        let source_A = SineWave::new(440.0).take_duration(Duration::from_secs_f32(0.5)).amplify(0.20);
        let source_Csharp = SineWave::new(554.37).take_duration(Duration::from_secs_f32(0.5)).amplify(0.20);
        let source_E = SineWave::new(659.25).take_duration(Duration::from_secs_f32(0.5)).amplify(0.20);
        sink.append(source_A);
        sink.append(source_Csharp);
        sink.append(source_E);

        // The sound plays in a separate thread. This call will block the current thread until the sink
        // has finished playing all its queued sounds.
        sink.sleep_until_end();
    }
    
    #[allow(deprecated)]
    el.run(move |event, _, control_flow| {
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
                    if state == ElementState::Pressed {
                        handle_click(last_postition, state, button, &mut tree);
                        //send_request();
                        play_sound();
                    }
                }
                _ => (),
            },
            Event::RedrawRequested(_) => {}
            _ => (),
        }

        handle_redraw(env.surface.canvas(), &mut tree);
        env.surface.canvas().flush();
        env.windowed_context.swap_buffers().unwrap();
    });
}
