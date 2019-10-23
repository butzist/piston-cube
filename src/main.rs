mod pipeline;
use pipeline::ObjectPipeline;

mod model;
use model::Drawable;

use std::sync::{Arc, Mutex};

//----------------------------------------

fn main() {
    use camera_controllers::{FirstPerson, FirstPersonSettings};
    use piston_window::*;

    let opengl = OpenGL::V3_2;

    let mut window: PistonWindow = WindowSettings::new("piston: cube", [800, 600])
        .exit_on_esc(true)
        .samples(4)
        .graphics_api(opengl)
        .build()
        .unwrap();
    window.set_capture_cursor(true);
    window.set_max_fps(60);

    let ref mut factory = window.factory.clone();

    let mut first_person = FirstPerson::new([0.0, 0.0, 4.0], FirstPersonSettings::keyboard_wasd());
    let pipeline = Arc::new(Mutex::new(ObjectPipeline::new(&window, opengl)));
    let mut scene = model::Scene::new(pipeline.clone(), factory);
    let mut cube = model::Cube::new(pipeline.clone(), factory);
    let mut cube2 = cube.clone_to([2.0, 1.0, -3.0]);

    while let Some(e) = window.next() {
        first_person.event(&e);
        cube.update();
        cube2.update();

        window.draw_3d(&e, |window| {
            let args = e.render_args().unwrap();
            let camera = first_person.camera(args.ext_dt);
            scene.update(window, &camera);

            window
                .encoder
                .clear(&window.output_color, [0.3, 0.3, 0.3, 1.0]);
            window.encoder.clear_depth(&window.output_stencil, 1.0);

            scene.draw(window);
            cube.draw(window);
            cube2.draw(window);
        });
    }
}
