pub mod objects;

use std::{ffi::c_void, ptr::null};

use gl::types::{GLint, GLuint};

pub struct Geometry {
    pub vertex_id: GLuint,
    pub vertex_attr_id: GLuint,
    pub uv_id: GLuint,
    pub uv_attr_id: GLuint,
    pub index_id: GLuint,

    pub border_color_id: GLuint,
    pub texture_id: GLuint,
    pub vertex_idx_id: GLuint,
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
            texture_id: id,
            vertex_idx_id: id,
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
    pub fn add_vertex_idx(&mut self, vertex_idx: &Vec<f32>) {
        unsafe {
            gl::GenBuffers(1, &mut self.vertex_idx_id);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vertex_idx_id);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertex_idx.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                vertex_idx.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );

            // gl::GenVertexArrays(1, &mut self.uv_attr_id);
            // gl::BindVertexArray(self.uv_attr_id);
            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(
                2,
                1,
                gl::FLOAT,
                gl::FALSE,
                (1 * std::mem::size_of::<f32>()) as GLint,
                null(),
            );
        }
    }

    pub fn add_texture(&mut self, tex: &Vec<f32>, width: i32, height: i32) {
        unsafe {
            gl::GenTextures(1, &mut self.texture_id);
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA32F as i32,
                width as i32,
                height as i32,
                0,
                gl::RGBA,
                gl::FLOAT,
                tex.as_ptr() as *const c_void,
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::GenerateMipmap(gl::TEXTURE_2D);
        };
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
