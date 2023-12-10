extern crate glfw;
extern crate gl;

use std::ffi::{CString, c_char};

use glfw::{Action, Context, Key, fail_on_errors, PWindow, Glfw, InitError, WindowEvent, GlfwReceiver, ffi::{glfwTerminate, glfwSwapBuffers, glfwInit, glfwWindowHint, GLFWwindow, glfwCreateWindow, glfwMakeContextCurrent, glfwSetFramebufferSizeCallback, glfwGetProcAddress, glfwPollEvents, glfwGetFramebufferSize, glfwWindowShouldClose}};

pub enum ShaderType {
    VERTEX,
    FRAGMENT,
    GEOMETRY,
}

pub struct Komrend {
    window: *mut GLFWwindow,
    old_size: (i32, i32),
    pub should_close: bool,
}

extern "C" fn framebuffer_size_callback(win: *mut GLFWwindow, w: i32, h: i32) {

}

pub fn init(w: u32, h: u32, title: &str) -> Result<Komrend, InitError> {
    unsafe {
        glfwInit();
        glfwWindowHint(glfw::ffi::CONTEXT_VERSION_MAJOR, 3);
        glfwWindowHint(glfw::ffi::CONTEXT_VERSION_MINOR, 3);
        glfwWindowHint(glfw::ffi::OPENGL_PROFILE, glfw::ffi::OPENGL_CORE_PROFILE);

        let win = glfwCreateWindow(w.try_into().unwrap(), h.try_into().unwrap(), std::ffi::CString::new(title).unwrap().as_ptr() as *const core::ffi::c_char, 0 as *mut glfw::ffi::GLFWmonitor, 0 as *mut glfw::ffi::GLFWwindow);
        // process window error
        //
        glfwMakeContextCurrent(win);
        glfwSetFramebufferSizeCallback(win, Some(framebuffer_size_callback));
        gl::load_with(|s| glfwGetProcAddress(std::ffi::CString::new(s).unwrap().as_ptr() as *const core::ffi::c_char));
        gl::Viewport::load_with(|s| glfwGetProcAddress(std::ffi::CString::new(s).unwrap().as_ptr() as *const core::ffi::c_char));

        gl::Viewport(0, 0, w as i32, h as i32);

    let rend = Komrend {
        window: win,
        old_size: (0, 0),
        should_close: false,
    };
    println!("Komrend successfully initialized");

    Ok(rend)
    }
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
    let dstr = unsafe {CString::from_vec_unchecked(data) };
    let dcstr = dstr.as_c_str();
    unsafe {
    let sh = gl::CreateShader(match ty {
         ShaderType::VERTEX => gl::VERTEX_SHADER,
         ShaderType::FRAGMENT => gl::FRAGMENT_SHADER,
         ShaderType::GEOMETRY => gl::GEOMETRY_SHADER,
    });
    gl::ShaderSource(sh, 1, &dcstr.as_ptr(), std::ptr::null());
    gl::CompileShader(sh);
    let mut success: i32 = 0;
    gl::GetShaderiv(sh, gl::COMPILE_STATUS, &mut success);
    if success != 1 {
        let mut buffer = Vec::<u8>::with_capacity(512);
        buffer.extend([b' '].iter().cycle().take(512));
        let infolog: CString = CString::from_vec_unchecked(buffer);
        gl::GetShaderInfoLog(sh, 512, 0 as *mut i32, infolog.as_ptr() as *mut c_char);
        println!("Error compiling shader: {}", infolog.to_str().unwrap());
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, infolog.to_str().unwrap()));
    }
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
        let mut success: i32 = 0;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
        if success != 1 {
            let mut buffer = Vec::<u8>::with_capacity(512);
            buffer.extend([b' '].iter().cycle().take(512));
            let infolog = CString::from_vec_unchecked(buffer);
            gl::GetProgramInfoLog(program, 512, 0 as *mut i32, infolog.as_ptr() as *mut c_char);
            println!("Error linking program: {}", infolog.to_str().unwrap());
        }
        program
    }
}

impl Komrend {
    pub fn update(&mut self) {
        unsafe {

        glfwPollEvents();
        //let size = glfwGetFramebufferSize(self.window, width, height)
        //if self.old_size != size {
        //    
        //    self.old_size = size;
        //}
        self.should_close = glfwWindowShouldClose(self.window) == 1;

        }
    }
    pub fn finish(&mut self) {
        unsafe { glfwSwapBuffers(self.window); };
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
        const VERTS: [f32; 9] = [
            -0.5, -0.5, 0.0,
            0.5, -0.5, 0.0,
            0.0, 0.5, 0.0,
        ];
        unsafe {
            gl::GenBuffers(1, &mut vbo);
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            println!("Size: {}", (std::mem::size_of::<f32>() * 9) as isize);
            gl::BufferData(gl::ARRAY_BUFFER, (std::mem::size_of::<f32>() * 9) as isize, VERTS.as_ptr() as *const _, gl::STATIC_DRAW);
            let program = create_shader_program(load_shader("resources/shaders/default.vs", ShaderType::VERTEX).unwrap(), load_shader("resources/shaders/default.fs", ShaderType::FRAGMENT).unwrap());
            gl::UseProgram(program);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * std::mem::size_of::<f32>() as i32, 0 as *const _);
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
    pub fn terminate(&mut self) {
        unsafe {glfwTerminate(); };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_test() {
        
    }
}
