pub mod objects;

use std::ptr::null;

use gl::types::{GLint, GLuint};

pub struct Geometry {
    pub vertex_id: GLuint,
    pub vertex_attr_id: GLuint,
    pub uv_id: GLuint,
    pub uv_attr_id: GLuint,
    pub index_id: GLuint,

    pub border_color_id: GLuint,
}

impl Drop for Geometry {
    fn drop(&mut self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::DeleteBuffers(1, &self.vertex_id);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
            gl::DeleteBuffers(1, &self.index_id);

            gl::BindVertexArray(0);
            gl::DeleteVertexArrays(1, &self.vertex_attr_id);

            gl::BindVertexArray(1);
            gl::DeleteVertexArrays(1, &self.uv_attr_id);
        }
    }
}

impl Geometry {
    pub fn new() -> Self {
        let id: GLuint = 0;
        Geometry {
            vertex_id: id,
            vertex_attr_id: id,
            uv_id: id,
            uv_attr_id: id,
            index_id: id,
            border_color_id: id,
        }
    }

    pub fn add_vertex(&mut self, vertices: &Vec<f32>) {
        unsafe {
            gl::GenBuffers(1, &mut self.vertex_id);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vertex_id);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                vertices.as_ptr() as *const gl::types::GLvoid,
                gl::DYNAMIC_DRAW,
            );

            gl::GenVertexArrays(1, &mut self.vertex_attr_id);
            gl::BindVertexArray(self.vertex_attr_id);
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                2,
                gl::FLOAT,
                gl::FALSE,
                (2 * std::mem::size_of::<f32>()) as GLint,
                null(),
            );
        }
    }

    pub fn add_uv(&mut self, uvs: &Vec<f32>) {
        unsafe {
            gl::GenBuffers(1, &mut self.uv_id);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.uv_id);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (uvs.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                uvs.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );

            // gl::GenVertexArrays(1, &mut self.uv_attr_id);
            // gl::BindVertexArray(self.uv_attr_id);
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                (2 * std::mem::size_of::<f32>()) as GLint,
                null(),
            );
        }
    }

    pub fn add_border_color(&mut self, border_color: &Vec<f32>) {
        unsafe {
            gl::GenBuffers(1, &mut self.border_color_id);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.border_color_id);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (border_color.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                border_color.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );

            // gl::GenVertexArrays(1, &mut self.uv_attr_id);
            // gl::BindVertexArray(self.uv_attr_id);
            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(
                2,
                4,
                gl::FLOAT,
                gl::FALSE,
                (4 * std::mem::size_of::<f32>()) as GLint,
                null(),
            );
        }
    }

    pub fn add_index(&mut self, indices: &Vec<u32>) {
        unsafe {
            gl::GenBuffers(1, &mut self.index_id);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.index_id);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr,
                indices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
        }
    }
}
