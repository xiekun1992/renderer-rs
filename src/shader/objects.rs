use std::{
    collections::HashMap,
    ffi::{CStr, CString},
    ptr::{null, null_mut},
};

use gl::{
    types::{GLchar, GLenum, GLint, GLuint},
    UseProgram,
};

/// An OpenGL Shader (of the graphics pipeline)
pub struct Shader {
    id: GLuint,
}

impl Shader {
    pub fn from_source(source: &CStr, kind: GLenum) -> Result<Self, String> {
        let id = unsafe { gl::CreateShader(kind) };
        unsafe {
            gl::ShaderSource(id, 1, &source.as_ptr(), null());
            gl::CompileShader(id);
        }

        let mut success: GLint = 1;
        unsafe {
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        }

        if success == 0 {
            // An error occured
            let mut len: GLint = 0;
            unsafe {
                gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl::GetShaderInfoLog(id, len, null_mut(), error.as_ptr() as *mut GLchar);
            }

            return Err(error.to_string_lossy().into_owned());
        }

        Ok(Shader { id })
    }

    pub fn id(&self) -> GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

/// An OpenGL Program, a sequence of shaders calls.
pub struct Program<'a> {
    pub id: GLuint,
    pub uniform_viewport: GLint,
    pub uniform: HashMap<&'a str, GLint>,
    pub attribute: HashMap<&'a str, GLint>,
}

impl<'a> Program<'a> {
    fn from_shaders(shaders: &[Shader]) -> Result<Self, String> {
        let id = unsafe { gl::CreateProgram() };

        for shader in shaders {
            unsafe {
                gl::AttachShader(id, shader.id());
            }
        }

        unsafe {
            gl::LinkProgram(id);
        }

        let mut success: GLint = 1;
        unsafe {
            gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            // An error occured
            let mut len: GLint = 0;
            unsafe {
                gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl::GetProgramInfoLog(id, len, null_mut(), error.as_ptr() as *mut GLchar);
            }

            return Err(error.to_string_lossy().into_owned());
        }

        for shader in shaders {
            unsafe {
                gl::DetachShader(id, shader.id());
            }
        }

        Ok(Program {
            id,
            uniform_viewport: 0,
            uniform: HashMap::new(),
            attribute: HashMap::new(),
        })
    }

    pub fn set(&mut self) {
        unsafe {
            UseProgram(self.id);

            #[rustfmt::skip]
            let attributes = [
                "viewport",

                "left", "top", "right", "bottom",

                "color", "font-size", "font-style", "font-weight",

                "border-top-width", "border-top-color", "border-top-style",
                "border-right-width", "border-right-color", "border-right-style",
                "border-bottom-width", "border-bottom-color", "border-bottom-style",
                "border-left-width", "border-left-color", "border-left-style",
    
                "padding-top", "padding-right", "padding-bottom", "padding-left",
    
                "margin-top", "margin-right", "margin-bottom", "margin-left",
            ];
            attributes.iter().for_each(|str| {
                let n = CString::new(*str).unwrap();
                self.attribute
                    .insert(*str, gl::GetAttribLocation(self.id, n.as_ptr()));
            });

            #[rustfmt::skip]
            let uniforms = [];

            uniforms.iter().for_each(|str| {
                let n = CString::new(*str).unwrap();
                self.uniform
                    .insert(*str, gl::GetUniformLocation(self.id, n.as_ptr()));
            });
        }
    }
}

impl<'a> Drop for Program<'a> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    buffer.extend([b' '].iter().cycle().take(len));
    unsafe { CString::from_vec_unchecked(buffer) }
}

pub fn create_program() -> Result<Program<'static>, &'static str> {
    let vert_shader = Shader::from_source(
        &CString::new(include_str!(".vert")).unwrap(),
        gl::VERTEX_SHADER,
    )
    .unwrap();
    let frag_shader = Shader::from_source(
        &CString::new(include_str!(".frag")).unwrap(),
        gl::FRAGMENT_SHADER,
    )
    .unwrap();

    let shader_program = Program::from_shaders(&[vert_shader, frag_shader]).unwrap();

    Ok(shader_program)
}

///// OpenGL Vertex Buffer Object
// pub struct Vbo {
//     pub id: GLuint,
// }

// impl Drop for Vbo {
//     fn drop(&mut self) {
//         self.unbind();
//         self.delete();
//     }
// }

// impl Vbo {
//     pub fn gen() -> Self {
//         let mut id: GLuint = 0;
//         unsafe {
//             gl::GenBuffers(1, &mut id);
//         }
//         Vbo { id }
//     }

//     pub fn set(&self, data: &Vec<f32>) {
//         self.bind();
//         self.data(data);
//     }

//     fn data(&self, vertices: &Vec<f32>) {
//         unsafe {
//             gl::BufferData(
//                 gl::ARRAY_BUFFER,
//                 (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
//                 vertices.as_ptr() as *const gl::types::GLvoid,
//                 gl::DYNAMIC_DRAW,
//             );
//         }
//     }

//     fn bind(&self) {
//         unsafe {
//             gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
//         }
//     }

//     fn unbind(&self) {
//         unsafe {
//             gl::BindBuffer(gl::ARRAY_BUFFER, 0);
//         }
//     }

//     fn delete(&self) {
//         unsafe {
//             gl::DeleteBuffers(1, &self.id);
//         }
//     }
// }

// /// OpenGL Index Buffer Object
// pub struct Ibo {
//     pub id: GLuint,
// }

// impl Drop for Ibo {
//     fn drop(&mut self) {
//         self.unbind();
//         self.delete();
//     }
// }

// impl Ibo {
//     pub fn gen() -> Self {
//         let mut id: GLuint = 0;
//         unsafe {
//             gl::GenBuffers(1, &mut id);
//         }
//         Ibo { id }
//     }

//     pub fn set(&self, data: &Vec<u32>) {
//         self.bind();
//         self.data(data);
//     }

//     fn data(&self, indices: &Vec<u32>) {
//         unsafe {
//             gl::BufferData(
//                 gl::ELEMENT_ARRAY_BUFFER,
//                 (indices.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr,
//                 indices.as_ptr() as *const gl::types::GLvoid,
//                 gl::DYNAMIC_DRAW,
//             );
//         }
//     }

//     fn bind(&self) {
//         unsafe {
//             gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id);
//         }
//     }

//     fn unbind(&self) {
//         unsafe {
//             gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
//         }
//     }

//     fn delete(&self) {
//         unsafe {
//             gl::DeleteBuffers(1, &self.id);
//         }
//     }
// }

// /// OpenGL Vertex Array Object
// pub struct Vao {
//     pub id: GLuint,
// }

// impl Drop for Vao {
//     fn drop(&mut self) {
//         self.unbind();
//         self.delete();
//     }
// }

// impl Vao {
//     pub fn gen() -> Self {
//         let mut id: GLuint = 0;
//         unsafe {
//             gl::GenVertexArrays(1, &mut id);
//         }
//         Vao { id }
//     }

//     pub fn set(&self) {
//         self.bind();
//         self.setup();
//     }

//     fn setup(&self) {
//         unsafe {
//             gl::EnableVertexAttribArray(0);
//             gl::VertexAttribPointer(
//                 0,
//                 2,
//                 gl::FLOAT,
//                 gl::FALSE,
//                 (2 * std::mem::size_of::<f32>()) as GLint,
//                 null(),
//             );
//         }
//     }

//     fn bind(&self) {
//         unsafe {
//             gl::BindVertexArray(self.id);
//         }
//     }

//     fn unbind(&self) {
//         unsafe {
//             gl::BindVertexArray(0);
//         }
//     }

//     fn delete(&self) {
//         unsafe {
//             gl::DeleteVertexArrays(1, &self.id);
//         }
//     }
// }
