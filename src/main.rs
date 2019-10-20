mod model;
use model::Drawable;

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
    let mut cube = model::Cube::new(factory, opengl);

    while let Some(e) = window.next() {
        first_person.event(&e);
        cube.update();

        window.draw_3d(&e, |window| {
            let args = e.render_args().unwrap();
            let camera = first_person.camera(args.ext_dt);

            window
                .encoder
                .clear(&window.output_color, [0.3, 0.3, 0.3, 1.0]);
            window.encoder.clear_depth(&window.output_stencil, 1.0);

            cube.draw(window, &camera);
        });
    }
}
