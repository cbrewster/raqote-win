use minifb::{Window, WindowOptions, Scale, Key, MouseMode};
use raqote::{DrawTarget, SolidSource, PathBuilder, Source, DrawOptions};

const WIDTH: usize = 600;
const HEIGHT: usize = 400;

fn main() {
    let mut window = Window::new("RAQOTE WINDOW!", WIDTH, HEIGHT, WindowOptions {
        resize: true,
        scale: Scale::X1,
        ..WindowOptions::default()
    }).unwrap();

    let mut draw_target = DrawTarget::new(WIDTH as i32, HEIGHT as i32);

    let (mut width, mut height) = (WIDTH, HEIGHT);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let (new_width, new_height) = window.get_size();
        // Window Resizing is currently broken.
        if new_width != width || new_height != height {
            draw_target = DrawTarget::new(new_width as i32, new_height as i32);
            width = new_width;
            height = new_height;
            dbg!(width, height);
        }

        draw_target.clear(SolidSource {r: 0xff, g: 0xff, b: 0xff, a: 0xff});

        if let Some((x, y)) = window.get_mouse_pos(MouseMode::Discard) {
            let mut pb = PathBuilder::new();
            pb.rect(x - 10.0, y - 10.0, 20.0, 20.0);
            let path = pb.finish();

            let source = Source::Solid(SolidSource {r: 0xff, g: 0x00, b: 0x00, a: 0xff});

            draw_target.fill(&path, &source, &DrawOptions::default());
        }


        window.update_with_buffer(draw_target.get_data()).unwrap();
    }
}