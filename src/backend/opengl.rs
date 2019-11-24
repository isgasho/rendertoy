/*use std::sync::Mutex;

struct GlContext {
    gl: gl::Gl,
    window: Option<GlutinContext>,
}

lazy_static! {
    static ref OPENGL: Mutex<Option<GlContext>> = { Mutex::new(None) };
}

pub fn set_global_gl_context(gl: gl::Gl, window: GlutinContext) {
    *OPENGL.lock().unwrap() = Some(GlContext {
        gl,
        window: Some(window),
    });
}

pub fn with_gl<F, R>(f: F) -> R
where
    F: FnOnce(&gl::Gl) -> R,
{
    with_gl_and_context(|gl, _| f(gfx))
}

pub fn with_gl_and_context<F, R>(f: F) -> R
where
    F: FnOnce(&gl::Gl, &GlutinCurrentContext) -> R,
{
    let mut opengl = OPENGL.lock().unwrap();
    let opengl = opengl.as_mut().unwrap();

    let window = unsafe {
        opengl
            .window
            .take()
            .unwrap()
            .make_current()
            .expect("make_current failed")
    };

    let res = f(&opengl.gl, &window);

    let window = unsafe { window.make_not_current().expect("make_not_current failed") };
    opengl.window = Some(window);
    res
}
*/