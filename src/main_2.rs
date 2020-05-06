use raylib::consts::BlendMode;
use raylib::consts::KeyboardKey;
use raylib::core::texture::Image;
use raylib::ffi::TextureFilterMode::FILTER_POINT;
use raylib::ffi::{DrawRectangle, GetFrameTime};
use raylib::prelude::*;
fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640 as i32, 480 as i32)
        .title("project_s")
        .resizable()
        .vsync()
        .build();

    rl.set_target_fps(30);

    let mut draw_buffer = rl
        .load_render_texture(&thread, map.width * 32, map.height * 32)
        .expect("could not create texture");
    draw_buffer.texture_mut().set_texture_filter(FILTER_POINT);

    //draw into the buffer
    {
        let mut gen = rl.begin_drawing(&thread);
        let mut gen = gen.begin_texture_mode(&mut draw_buffer);
        for layer in &map.layers {
            for (y, y_item) in layer.tiles.iter().enumerate() {
                for (x, tile) in y_item.iter().enumerate() {
                    let tile = tile.gid;
                    let pos = Vector2::new(32.0 * x as f32, 32.0 * y as f32);
                    //println!("x: {} y: {} || px: {} py: {}", x, y, pos.x, pos.y);
                    rl.drwa_texture_rec();

                    raylib::ffi::DrawTextureRec(
                        draw_buffer,
                        Rectangle {
                            x: 0,
                            y: 0,
                            width: 32,
                            height: 32,
                        },
                        Vector2 { x: 0, y: 0 },
                        COLOR::WHITE,
                    );
                }
            }
        }
    }
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RED);
        d.draw_texture(draw_buffer.texture(), 0, 0, Color::WHITE);

        d.draw_texture_rec(
            draw_buffer.texture(),
            Rectangle::new(
                0.0,
                0.0,
                draw_buffer.texture().width as f32,
                -draw_buffer.texture().height as f32,
            ),
            Vector2::new(0.0, 0.0),
            Color::WHITE,
        );
        //dbg!(draw_buffer.texture().width());
        //dbg!(draw_buffer.texture().height());
    }
}
