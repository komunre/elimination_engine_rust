extern crate komrend;

fn main() {
    println!("Hello, world!");

    // OPENGL STUFF

    let mut rend = komrend::init(800, 600, "hi").unwrap();
    let (vao, vbo, program) = rend.do_triangle();
    while !rend.should_close {
        rend.update();
        rend.clear();
        rend.draw_vao(program, vao);
        rend.finish();
    }


    // NON-OPENGL, GAME
    
}
