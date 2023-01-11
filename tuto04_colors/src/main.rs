#[macro_use]
extern crate glium;

fn main() {
    //初期設定
    #[allow(unused_imports)]
    use glium::{glutin, Surface};

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }

    implement_vertex!(Vertex, position);

    //形状を書く
    let vertex1 = Vertex { position: [-0.5, -0.5] };
    let vertex2 = Vertex { position: [ 0.0,  0.5] };
    let vertex3 = Vertex { position: [ 0.5, -0.25] };
    let shape = vec![vertex1, vertex2, vertex3];

    //ビデオカードのメモリにアップロード
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);


    //GLSL頂点シェーダー
    let vertex_shader_src = r#"
        #version 140

        in vec2 position;
        out vec2 my_attr;   // new attribute

        uniform mat4 matrix;
        
        void main() {
            my_attr = position;     // outに値を代入する必要がある
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
    "#;

    //GLSLフラグメントシェーダー(ピクセルシェーダー)
    let fragment_shader_src = r#"
        #version 140

        in vec2 my_attr;
        out vec4 color;
        
        void main() {
            color = vec4(my_attr, 1.0, 1.0);    // 4つの値のうち、2つをmy_attrから取得している
        }
    "#;

    //シェーダーのソースコードをgliumに送信
    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
    
    let mut t: f32 = -0.5;
    //window展開
    event_loop.run(move |event, _, control_flow| {
        

        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        //！！！ここに処理を書く！！！//

        t += t.sin() + 1.01;

        let uniforms = uniform! {
            matrix: [
                [ t.cos(), t.sin(), 0.0, 0.0],
                [-t.sin(), t.cos(), 0.0, 0.0],
                [ 0.0, 0.0, 1.0, 0.0],
                [ 0.0, 0.0, 0.0, 1.0f32],
            ]
        };
        //描画開始
        let mut target = display.draw();
        target.clear_color(1.0, 1.0, 1.0, 1.0);
        target.draw(&vertex_buffer, &indices, &program, &uniforms,
            &Default::default()).unwrap();
        
        target.finish().unwrap();
        //描画終了
    });
}
