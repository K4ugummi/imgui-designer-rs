use gl;
use glfw::{Action, Context, Key};
use imgui::{im_str, FontGlyphRange, ImFontConfig, ImGui, ImGuiCol, ImGuiCond, ImVec2, ImVec4};
use imgui_glfw_rs::ImguiGLFW;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    let (mut window, events) = glfw
        .with_primary_monitor(|glfw, _| {
            glfw.create_window(1366, 768, "Carambolage", glfw::WindowMode::Windowed)
        })
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_all_polling(true);
    window.set_cursor_mode(glfw::CursorMode::Normal);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
    unsafe {
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LESS);
        gl::ClearColor(0.2, 0.2, 0.2, 1.0);
    }
    let mut imgui = init_imgui();

    let mut imgui_glfw = ImguiGLFW::new(&mut imgui);

    let imgui_renderer =
        imgui_opengl_renderer::Renderer::new(&mut imgui, |s| window.get_proc_address(s) as _);

    while !window.should_close() {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        {
            let ui = imgui_glfw.frame(&mut window, &mut imgui);

            ui.show_demo_window(&mut true);

            ui.show_default_style_editor();

            imgui_renderer.render(ui);
        };

        window.swap_buffers();
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            imgui_glfw.handle_event(&mut imgui, &event);
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                }
                _ => {}
            }
        }
    }
}

pub fn init_imgui() -> ImGui {
    let mut imgui = ImGui::init();

    imgui.set_ini_filename(None);

    let font_size = 18.0 as f32;

    imgui.fonts().add_default_font_with_config(
        ImFontConfig::new()
            .oversample_h(1)
            .pixel_snap_h(true)
            .size_pixels(font_size),
    );

    imgui.fonts().add_font_with_config(
        include_bytes!("../res/ProFontWindows.ttf"),
        ImFontConfig::new()
            .merge_mode(true)
            .oversample_h(1)
            .pixel_snap_h(true)
            .size_pixels(font_size)
            .rasterizer_multiply(1.75),
        &FontGlyphRange::default(),
    );

    imgui.set_font_global_scale(1.0);

    imgui
}
