extern crate glfw;
extern crate gl;

use glfw::{Action, Context, Key, fail_on_errors, PWindow, Glfw, InitError, WindowEvent, GlfwReceiver};

pub enum ShaderType {
    VERTEX,
    FRAGMENT,
    GEOMETRY,
}

pub struct Komrend {
    glfw_inst: Glfw,
    window: PWindow,
    events: GlfwReceiver<(f64, WindowEvent)>,
    old_size: (i32, i32),
    pub should_close: bool,
}

pub fn init(w: u32, h: u32, title: &str) -> Result<Komrend, InitError> {
    let mut glfw_i = match glfw::init(glfw::fail_on_errors!()) {
        Ok(g) => g,
        Err(e) => return Err(e)
    };
    let (mut win, ev) = match glfw_i.create_window(w, h, title, glfw::WindowMode::Windowed) {
        Some(win) => win,
        None => return Err(InitError::Internal),
    };

    win.set_key_polling(true);
    win.make_current();

    gl::load_with(|s| glfw_i.get_proc_address_raw(s));
    gl::Viewport::load_with(|s| glfw_i.get_proc_address_raw(s));

    unsafe {gl::Viewport(0, 0, w as i32, h as i32); };

    let rend = Komrend {
        glfw_inst: glfw_i,
        window: win,
        events: ev,
        old_size: (0, 0),
        should_close: false,
    };
    println!("Komrend successfully initialized");

    Ok(rend)
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        }
        _ => {}
    }
}

pub fn load_shader(path: &str, ty: ShaderType) -> Result<u32, std::io::Error> {
    println!("Loading shader of path: {}", path);
    let data = std::fs::read(path)?;
    let mut d2 = Vec::<*const i8>::new();
    let mut d3 = Vec::<i8>::new();
    for d in data {
        d3.push(d as i8);
    }
    d2.push(d3.as_ptr());
    unsafe {
    let sh = gl::CreateShader(match ty {
         ShaderType::VERTEX => gl::VERTEX_SHADER,
         ShaderType::FRAGMENT => gl::FRAGMENT_SHADER,
         ShaderType::GEOMETRY => gl::GEOMETRY_SHADER,
    });
    gl::ShaderSource(sh, 1, d2.as_ptr(), std::ptr::null());
    gl::CompileShader(sh);
    Ok(sh)
    }
}

pub fn create_shader_program(v: u32, f: u32) -> u32 {
    println!("Creating shader program: {:#?} -- {:#?}", v, f);
    unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, v);
        gl::AttachShader(program, f);
        gl::LinkProgram(program);
        program
    }
}

impl Komrend {
    pub fn update(&mut self) {
        self.glfw_inst.poll_events();
        for (_, event) in glfw::flush_messages(&self.events) {
            handle_window_event(&mut self.window, event);
        }
        let size = self.window.get_framebuffer_size();
        if self.old_size != size {
            
            self.old_size = size;
        }
        self.should_close = self.window.should_close();
    }
    pub fn finish(&mut self) {
        self.window.swap_buffers();
    }
    pub fn clear(&self) {
        unsafe {
        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }
    pub fn do_triangle(&self) -> (u32, u32, u32) {
        let mut vbo: u32 = 0;
        let mut vao: u32 = 0;
        const VERTS: [f64; 9] = [
            -0.5, -0.5, 0.0,
            0.5, -0.5, 0.0,
            0.0, 0.5, 0.0,
        ];
        unsafe {
            gl::GenBuffers(1, &mut vbo);
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(gl::ARRAY_BUFFER, (std::mem::size_of::<f64>() * 9) as isize, VERTS.as_ptr() as *const _, gl::STATIC_DRAW);
            let program = create_shader_program(load_shader("resources/shaders/default.vs", ShaderType::VERTEX).unwrap(), load_shader("resources/shaders/default.fs", ShaderType::FRAGMENT).unwrap());
            gl::UseProgram(program);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * std::mem::size_of::<f64>() as i32, 0 as *const _);
            gl::EnableVertexAttribArray(0);
            (vao, vbo, program)
        }
    }
    pub fn draw_vao(&self, program: u32, vao: u32) {
        unsafe {
            gl::UseProgram(program);
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_test() {
        
    }
}
