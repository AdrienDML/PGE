use gl;
use gl::types::*;
use crate::Vbuf;

pub struct VbElements {
    etype : GLenum,
    count : GLint,
    normalized : GLboolean
}

impl VbElements {

    pub fn new(etype : GLenum, count : GLint, normalized : GLboolean) -> Self {
        Self {
            etype : etype,
            count : count,
            normalized : normalized
        }
    }

    pub fn get_type_size(etype : GLenum) -> GLuint {
        match etype {
            gl::FLOAT => 4,
            gl::UNSIGNED_INT => 4,
            gl::UNSIGNED_BYTE => 1,
            _ => 0,
        }
    }

}

pub struct VbLayout {
    elements : Vec<VbElements>,
    stride : GLsizei,
}

impl VbLayout {
    pub fn new() -> Self {
        Self {
            elements : Vec::new(),
            stride : 0,
        }
    }

    pub fn push_f64(&mut self, count : GLint) 
    {
        self.elements.push(VbElements::new(gl::FLOAT, count, gl::FALSE));
        self.stride += (count as GLuint * VbElements::get_type_size(gl::FLOAT)) as GLsizei;
    }

    pub fn push_u32(&mut self, count : GLint) 
    {
        self.elements.push(VbElements::new(gl::UNSIGNED_INT, count, gl::FALSE));
        self.stride += (count as GLuint * VbElements::get_type_size(gl::UNSIGNED_INT)) as GLsizei;
    }

    pub fn push_u8(&mut self, count : GLint) 
    {
        self.elements.push(VbElements::new(gl::UNSIGNED_BYTE, count, gl::FALSE));
        self.stride += (count as GLuint * VbElements::get_type_size(gl::UNSIGNED_BYTE)) as GLsizei;
    }
}


pub struct VArray {
    _id : GLuint
}

impl VArray {
    pub fn new() -> Self {
        let mut id : GLuint = 0;
        unsafe {gl::GenVertexArrays(1, &mut id)}
        Self {
            _id : id
        }
    }

    pub fn add_buffer(&self, vertex_buffer : &Vbuf, vertex_buffer_layout : &VbLayout) {
        self.bind();
        vertex_buffer.bind();
        let mut offset : GLuint = 0;
        let mut elems_count : GLuint = 0;
        for element in vertex_buffer_layout.elements.as_slice() {
            unsafe {
                gl::EnableVertexAttribArray(elems_count);
                gl::VertexAttribPointer(elems_count, element.count, element.etype, element.normalized, vertex_buffer_layout.stride, offset as *const GLvoid);
                elems_count += 1 as GLuint;
                offset += element.count as GLuint * VbElements::get_type_size(element.etype);
            }
        }

    }

    pub fn bind(&self) {
        unsafe {gl::BindVertexArray(self._id)};
    }

    pub fn unbind(&self) {
        unsafe {gl::BindVertexArray(0)};
    }
}
impl Drop for VArray {
    fn drop(&mut self){
        unsafe {gl::DeleteVertexArrays(1 as GLsizei, &self._id)};
    }
}