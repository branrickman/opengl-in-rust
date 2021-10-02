
//Following opengl rust tutorial by Nerijus Arlauskas at:
//http://nercury.github.io/rust/opengl/tutorial/2018/02/08/opengl-in-rust-from-scratch-00-setup.html

extern crate sdl2;
extern crate gl;
pub mod render_gl;

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);

    //create window and set attributes
    let window = video_subsystem
        .window("game", 900, 700)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    //generate an opengl context ?
    let _gl_context = window.gl_create_context().unwrap();
    let _gl = 
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    //set viewport and color used to clear the color buffers with glClear()
    unsafe {
        gl::Viewport(0, 0, 900, 700); 
        gl::ClearColor(0.3, 0., 0.5, 1.0);
    }

    //compile shaders
    use std::ffi::CString;
    let vert_shader = 
    render_gl::Shader::from_vert_source(&CString::new(include_str!("triangle.vert")).unwrap())
    .unwrap();

    let frag_shader = 
    render_gl::Shader::from_frag_source(&CString::new(include_str!("triangle.frag")).unwrap())
    .unwrap(); //include_str! macro embeds UTF-8 contents as &str

    //create shader program from the compiled vertex and fragment shaders
    let shader_program = render_gl::Program::from_shaders(&[vert_shader, frag_shader])
    .unwrap();

    shader_program.set_used();

    //define vertices
    let vertices: Vec<f32> = vec![
        -0.5, -0.5, 0.0,
         0.5, -0.5, 0.0,
         0.0,  0.5, 0.0
    ];

    //create vertex buffer object
    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
    }

    //add vertex data into VBO buffer
    unsafe { 
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            vertices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0); //unbind buffer
    }

    //create VAO
    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
    }

    unsafe {
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0, //index of the generic vertex attribute
            3, //number of components per generic vertex attribute
            gl::FLOAT, //data type
            gl::FALSE, //normalization-- int-to-float conversion?
            (3 * std::mem::size_of::<f32>()) as gl::types::GLint, //"stride", or byte offset between consecutive vertex attributes
            std::ptr::null(), //offset of first component ?
        );

        gl::BindBuffer(gl::ARRAY_BUFFER, 0); //unbind vertex buffer object
        gl::BindVertexArray(0); //Unbind vertex array object
    }

    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'main,
                _ => {},
            }
        }

        //render here
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        shader_program.set_used();
        unsafe {
            gl::BindVertexArray(vao);
            gl::DrawArrays(
                gl::TRIANGLES, //draw mode
                0, //starting index in arrays
                3, // number of indices to render
            )
        }

        window.gl_swap_window();
    }
}

