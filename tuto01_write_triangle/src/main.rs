#[macro_use]
extern crate glium;

fn main() {
    //初期設定

    use glium::{glutin, Surface};

    let mut event_loop = glutin::event_loop::EventLoop::new();
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

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    //GLSLフラグメントシェーダー(ピクセルシェーダー)
    let fragment_shader_src = r#"
        #version 140

        out vec4 color;
        
        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    //シェーダーのソースコードをgliumに送信
    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();



    //window展開
    event_loop.run(move |ev, _, control_flow| {
        
        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            _ => (),
        }
        //！！！ここに処理を書く！！！//

        //描画開始
        let mut target = display.draw();
        
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms,
            &Default::default()).unwrap();
        target.finish().unwrap();
        //描画終了
    });
}