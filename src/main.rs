use gfx;
use gfx::{
    gfx_impl_struct_meta, gfx_pipeline, gfx_pipeline_inner, gfx_vertex_struct,
    gfx_vertex_struct_meta,
};

gfx_vertex_struct!(Vertex {
    a_pos: [i8; 4] = "a_pos",
    a_tex_coord: [i8; 2] = "a_tex_coord",
});

impl Vertex {
    fn new(pos: [i8; 3], tc: [i8; 2]) -> Vertex {
        Vertex {
            a_pos: [pos[0], pos[1], pos[2], 1],
            a_tex_coord: tc,
        }
    }
}

gfx_pipeline!( pipe {
    vbuf: gfx::VertexBuffer<Vertex> = (),
    u_model_view_proj: gfx::Global<[[f32; 4]; 4]> = "u_model_view_proj",
    t_color: gfx::TextureSampler<[f32; 4]> = "t_color",
    out_color: gfx::RenderTarget<::gfx::format::Srgba8> = "o_Color",
    out_depth: gfx::DepthTarget<::gfx::format::DepthStencil> =
        gfx::preset::depth::LESS_EQUAL_WRITE,
});

//----------------------------------------

fn main() {
    use camera_controllers::{
        model_view_projection, CameraPerspective, FirstPerson, FirstPersonSettings,
    };
    use gfx::traits::*;
    use piston_window::*;
    use shader_version::glsl::GLSL;
    use shader_version::Shaders;

    let opengl = OpenGL::V3_2;

    let mut window: PistonWindow = WindowSettings::new("piston: cube", [800, 600])
        .exit_on_esc(true)
        .samples(4)
        .graphics_api(opengl)
        .build()
        .unwrap();
    window.set_capture_cursor(true);

    let ref mut factory = window.factory.clone();

    let vertex_data = vec![
        //top (0, 0, 1)
        Vertex::new([-1, -1, 1], [0, 0]),
        Vertex::new([1, -1, 1], [1, 0]),
        Vertex::new([1, 1, 1], [1, 1]),
        Vertex::new([-1, 1, 1], [0, 1]),
        //bottom (0, 0, -1)
        Vertex::new([1, 1, -1], [0, 0]),
        Vertex::new([-1, 1, -1], [1, 0]),
        Vertex::new([-1, -1, -1], [1, 1]),
        Vertex::new([1, -1, -1], [0, 1]),
        //right (1, 0, 0)
        Vertex::new([1, -1, -1], [0, 0]),
        Vertex::new([1, 1, -1], [1, 0]),
        Vertex::new([1, 1, 1], [1, 1]),
        Vertex::new([1, -1, 1], [0, 1]),
        //left (-1, 0, 0)
        Vertex::new([-1, 1, 1], [0, 0]),
        Vertex::new([-1, -1, 1], [1, 0]),
        Vertex::new([-1, -1, -1], [1, 1]),
        Vertex::new([-1, 1, -1], [0, 1]),
        //front (0, 1, 0)
        Vertex::new([-1, 1, -1], [0, 0]),
        Vertex::new([1, 1, -1], [1, 0]),
        Vertex::new([1, 1, 1], [1, 1]),
        Vertex::new([-1, 1, 1], [0, 1]),
        //back (0, -1, 0)
        Vertex::new([1, -1, 1], [0, 0]),
        Vertex::new([-1, -1, 1], [1, 0]),
        Vertex::new([-1, -1, -1], [1, 1]),
        Vertex::new([1, -1, -1], [0, 1]),
    ];

    let index_data: &[u16] = &[
        0, 1, 2, 2, 3, 0, // top
        4, 6, 5, 6, 4, 7, // bottom
        8, 9, 10, 10, 11, 8, // right
        12, 14, 13, 14, 12, 15, // left
        16, 18, 17, 18, 16, 19, // front
        20, 21, 22, 22, 23, 20, // back
    ];

    let (vbuf, slice) = factory.create_vertex_buffer_with_slice(&vertex_data, index_data);

    let target = "https://i.imgur.com/40VzkBZ.jpg";
    let mut buffer: Vec<u8> = vec![];
    reqwest::get(target).unwrap().copy_to(&mut buffer).unwrap();

    let image = ::image::load_from_memory(&buffer).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let (_, texture_view) = factory
        .create_texture_immutable_u8::<gfx::format::Rgba8>(
            gfx::texture::Kind::D2(
                image_dimensions.0 as u16,
                image_dimensions.1 as u16,
                gfx::texture::AaMode::Single,
            ),
            gfx::texture::Mipmap::Provided,
            &[&image],
        )
        .unwrap();

    let sinfo = gfx::texture::SamplerInfo::new(
        gfx::texture::FilterMethod::Bilinear,
        gfx::texture::WrapMode::Clamp,
    );

    let glsl = opengl.to_glsl();
    let pso = factory
        .create_pipeline_simple(
            Shaders::new()
                .set(GLSL::V1_50, include_str!("cube_150.glslv"))
                .get(glsl)
                .unwrap()
                .as_bytes(),
            Shaders::new()
                .set(GLSL::V1_50, include_str!("cube_150.glslf"))
                .get(glsl)
                .unwrap()
                .as_bytes(),
            pipe::new(),
        )
        .unwrap();

    let get_projection = |w: &PistonWindow| {
        let draw_size = w.window.draw_size();
        CameraPerspective {
            fov: 90.0,
            near_clip: 0.1,
            far_clip: 1000.0,
            aspect_ratio: (draw_size.width as f32) / (draw_size.height as f32),
        }
        .projection()
    };

    let mut projection = get_projection(&window);
    let mut first_person = FirstPerson::new([0.5, 0.5, 4.0], FirstPersonSettings::keyboard_wasd());

    let mut data = pipe::Data {
        vbuf: vbuf.clone(),
        u_model_view_proj: [[0.0; 4]; 4],
        t_color: (texture_view, factory.create_sampler(sinfo)),
        out_color: window.output_color.clone(),
        out_depth: window.output_stencil.clone(),
    };

    let start_time = std::time::SystemTime::now();

    while let Some(e) = window.next() {
        let t = start_time.elapsed().unwrap().as_millis() as f32 / 1000.0;
        let model = cgmath::Matrix4::from_axis_angle([1.0f32, 0.3, 0.3].into(), cgmath::Rad(t));
        first_person.event(&e);

        window.draw_3d(&e, |window| {
            let args = e.render_args().unwrap();

            window
                .encoder
                .clear(&window.output_color, [0.3, 0.3, 0.3, 1.0]);
            window.encoder.clear_depth(&window.output_stencil, 1.0);

            data.u_model_view_proj = model_view_projection(
                model.into(),
                first_person.camera(args.ext_dt).orthogonal(),
                projection,
            );
            window.encoder.draw(&slice, &pso, &data);
        });

        if let Some(_) = e.resize_args() {
            projection = get_projection(&window);
            data.out_color = window.output_color.clone();
            data.out_depth = window.output_stencil.clone();
        }
    }
}
