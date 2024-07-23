mod winsdl;
use std::time::Duration;

use gl::types::GLuint;
use shader::Geometry;
use winsdl::Winsdl;
mod shader;
use shader::objects::*;

use sdl2::event::{Event, WindowEvent};

struct Style {
    border_top_width: f32,
    border_top_color: (f32, f32, f32, f32),
    border_right_width: f32,
    border_right_color: (f32, f32, f32, f32),
    border_bottom_width: f32,
    border_bottom_color: (f32, f32, f32, f32),
    border_left_width: f32,
    border_left_color: (f32, f32, f32, f32),
}
struct Renderer {
    count: f32,
    geo: Geometry,
    indices: usize,
}
impl Renderer {
    pub fn new() -> Self {
        Renderer {
            count: 0.0,
            geo: Geometry::new(),
            indices: 0,
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
            2, 0, 1, 1, 3, 2, // 0, 1, 2, //
              // 2, 1, 3, //
        ];
        let uvs: Vec<f32> = vec![
            0.0, 1.0, //
            1.0, 1.0, //
            0.0, 0.0, //
            1.0, 0.0, //
        ];
        (vertices, indices, uvs)
    }
    pub fn add_rect(&mut self, x: f32, y: f32, width: f32, height: f32, style: Style) {
        let (mut vertices1, mut indices1, mut uvs1) = Renderer::rect_fan(x, y, width, height);
        self.indices += vertices1.len();
        // let (mut vertices2, mut indices2, mut uvs2) = Renderer::rect_fan(120.0, 8.0, 100.0, 100.0);

        let mut vertices = Vec::new();
        vertices.append(&mut vertices1);
        // vertices.append(&mut vertices2);
        self.geo.add_vertex(&vertices);

        let mut indices = Vec::new();
        indices.append(&mut indices1);
        // indices2.iter_mut().for_each(|x| *x += 4);
        // println!("{:?}", indices2);
        // indices.append(&mut indices2);
        self.geo.add_index(&indices);

        let mut uvs = Vec::new();
        uvs.append(&mut uvs1);
        // uvs.append(&mut uvs2);
        self.geo.add_uv(&uvs);

        #[rustfmt::skip]
        let colors: Vec<f32> = vec![
            style.border_top_color.0, style.border_top_color.1, style.border_top_color.2, style.border_top_color.3,
            style.border_right_color.0, style.border_right_color.1, style.border_right_color.2, style.border_right_color.3,
            style.border_bottom_color.0, style.border_bottom_color.1, style.border_bottom_color.2, style.border_bottom_color.3,
            style.border_left_color.0, style.border_left_color.1, style.border_left_color.2, style.border_left_color.3,
            style.border_top_width / height, style.border_right_width / width, style.border_bottom_width / height, style.border_left_width / width
            // 255.0, 0.0, 0.0, 255.0,    // 红色
            // 0.0, 255.0, 0.0, 255.0,    // 绿色
            // 0.0, 0.0, 255.0, 255.0,    // 蓝色
            // 255.0, 255.0, 0.0, 255.0, // 黄色
            // 0.25, 0.1, 0.25, 0.1, 

            // 255.0, 255.0, 0.0, 255.0, // 黄色
            // 0.0, 0.0, 255.0, 255.0,    // 蓝色
            // 0.0, 255.0, 0.0, 255.0,    // 绿色
            // 255.0, 0.0, 0.0, 255.0,    // 红色
            // 0.25, 0.25, 0.25, 0.25,
        ];
        // println!("{:?}", colors);
        self.geo.add_texture(&colors, 5, colors.len() as i32 / 5);

        let idx = self.count;
        let vertex_idx: Vec<f32> = vec![idx, idx, idx, idx];
        self.count += 1.0;
        // let vertex_idx: Vec<f32> = vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0];
        self.geo.add_vertex_idx(&vertex_idx);
    }
}

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

    let mut renderer = Renderer::new();
    let red = (255.0, 0.0, 0.0, 255.0);
    renderer.add_rect(
        10.0,
        10.0,
        100.0,
        40.0,
        Style {
            // background_color: (),
            border_top_color: red,    //(255.0, 0.0, 0.0, 255.0),
            border_right_color: red,  //(0.0, 255.0, 0.0, 255.0),
            border_bottom_color: red, //(0.0, 0.0, 255.0, 255.0),
            border_left_color: red,   //(255.0, 255.0, 0.0, 255.0),
            border_top_width: 4.0,
            border_right_width: 4.0,
            border_bottom_width: 4.0,
            border_left_width: 4.0,
        },
    );
    // renderer.add_rect(120.0, 8.0, 100.0, 100.0);

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
            // gl::BindTexture(gl::TEXTURE_2D, renderer.geo.texture_id);

            gl::DrawElements(
                gl::TRIANGLES,
                renderer.indices as i32,
                gl::UNSIGNED_INT,
                0 as *const _,
            );
        }
        winsdl.window.gl_swap_window();
        std::thread::sleep(Duration::from_millis(16));
    }
}
