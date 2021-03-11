use pge::*;
use std::path::Path;
use window::App;
use ressources::Ressources;
use render::VertexBuffer;
use render::IndexBuffer;
use render::VbLayout;
use render::VArray;
use render::ShaderProgram;
use render::GlObj;

use gl::types::*;

fn main() {
    let mut app = App::new("Test", 1080, 720);
    let vertices = vec![
        0.5, 0.5,  0.0, // top right
        0.5, -0.5, 0.0, // bottom right
        -0.5, -0.5, 0.0, // bottom left
        -0.5, 0.5, 0.0 // top left
    ];
    let indices = vec![ // note that we start from 0!
        0, 1, 3, // first triangle
        1, 2, 3    // second triangle
    ];
    // initialize the path for loading ressouce from
    let res = match Ressources::from_rel_path(Path::new("./")){
        Ok(res) => res,
        Err(_) => app.client_logger.fatal(&"Res path is wrong!".to_string()),
    };

    let vertex_array = VArray::new(&app.gl);
    let prog = ShaderProgram::from_res(&app.gl, &res, "tri").unwrap();
    let vertex_buffer = VertexBuffer::new(&app.gl);
    vertex_buffer.set_data::<f32>(&vertices);
    let mut vb_layout = VbLayout::new();
    vb_layout.push_f64(3 as GLint);
    
    vertex_array.add_buffer(&vertex_buffer, &vb_layout);
    let index_buffer = IndexBuffer::new(&app.gl);
    index_buffer.set_data::<u32>(&indices);

    let draw = || {
        prog.bind();
        vertex_array.draw_indexed(&index_buffer);
    };

    app.run(&draw);
}