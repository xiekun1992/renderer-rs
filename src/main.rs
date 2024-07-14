mod winsdl;
use std::time::Duration;

use gl::types::GLuint;
use shader::Geometry;
use winsdl::Winsdl;
mod shader;
use shader::objects::*;

use sdl2::event::{Event, WindowEvent};

fn main() {
    let mut w: usize = 800;
    let mut h: usize = 600;
    let mut winsdl: Winsdl = Winsdl::new(w, h).unwrap();
    unsafe {
        gl::Viewport(0, 0, w as i32, h as i32);
    }

    let mut program = create_program().unwrap();
    program.set();
    unsafe {
        gl::VertexAttrib4f(
            *program.attribute.get("viewport").unwrap() as GLuint,
            0.0,
            0.0,
            w as f32,
            h as f32,
        );
    }

    let (mut vertices1, mut indices1, mut uvs1) = rect_fan(8.0, 8.0, 100.0, 40.0);
    let (mut vertices2, mut indices2, mut uvs2) = rect_fan(120.0, 8.0, 100.0, 30.0);

    let mut geo = Geometry::new();
    let mut vertices = Vec::new();
    vertices.append(&mut vertices1);
    vertices.append(&mut vertices2);
    geo.add_vertex(&vertices);

    let mut indices = Vec::new();
    indices.append(&mut indices1);
    indices2.iter_mut().for_each(|x| *x += 4);
    // println!("{:?}", indices2);
    indices.append(&mut indices2);
    geo.add_index(&indices);

    let mut uvs = Vec::new();
    uvs.append(&mut uvs1);
    uvs.append(&mut uvs2);
    geo.add_uv(&uvs);

    'running: loop {
        for event in winsdl.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::Window {
                    timestamp: _,
                    window_id: _,
                    win_event,
                } => match win_event {
                    WindowEvent::Resized(ww, wh) => {
                        w = ww as usize;
                        h = wh as usize;
                        println!("{}, {}", w, h);
                        unsafe {
                            gl::Viewport(0, 0, ww, wh);
                            gl::VertexAttrib4f(
                                *program.attribute.get("viewport").unwrap() as GLuint,
                                0.0,
                                0.0,
                                w as f32,
                                h as f32,
                            );
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        unsafe {
            // gl::ClearColor(1., 1., 1., 1.0);
            gl::ClearColor(54. / 255., 159. / 255., 219. / 255., 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::DrawElements(
                gl::TRIANGLES,
                indices.len() as i32,
                gl::UNSIGNED_INT,
                0 as *const _,
            );
        }
        winsdl.window.gl_swap_window();
        std::thread::sleep(Duration::from_millis(16));
    }
}

fn rect_fan(x: f32, y: f32, width: f32, height: f32) -> (Vec<f32>, Vec<u32>, Vec<f32>) {
    let vertices: Vec<f32> = vec![
        x,            // / (w as f32),
        y,            // / (h as f32), //
        (x + width),  // / (w as f32),
        y,            // / (h as f32), //
        x,            // / (w as f32),
        (y + height), // / (h as f32), //
        (x + width),  // / (w as f32),
        (y + height), // / (h as f32), //
    ];
    let indices: Vec<u32> = vec![
        0, 1, 2, //
        2, 1, 3, //
    ];
    let uvs: Vec<f32> = vec![
        0.0, 1.0, //
        1.0, 1.0, //
        0.0, 0.0, //
        1.0, 0.0, //
    ];
    (vertices, indices, uvs)
}
