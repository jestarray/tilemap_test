use raylib::consts::BlendMode;
use raylib::consts::KeyboardKey;
use raylib::core::texture::Image;
use raylib::ffi::TextureFilterMode::FILTER_POINT;
use raylib::ffi::{DrawRectangle, GetFrameTime};
use raylib::prelude::*;
use std::convert::TryInto;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use tiled::parse;
use tiled::Map;
use tiled::*;

fn main() {
    let file = File::open(&Path::new("assets/mymap.tmx")).unwrap();
    let reader = BufReader::new(file);
    let map: Map = parse(reader).unwrap();
    //dbg!(&map.layers[0].tiles[0]);

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

    let floor_image = Image::load_image("assets/floors.png").expect("could not load floor image");
    let floor_texture = rl
        .load_texture_from_image(&thread, &floor_image)
        .expect("could not convert floor image to texture");

    //draw into the buffer
    {
        let mut gen = rl.begin_drawing(&thread);
        let mut gen = gen.begin_texture_mode(&mut draw_buffer);
        for layer in &map.layers {
            for (y, y_item) in layer.tiles.iter().enumerate() {
                for (x, tile) in y_item.iter().enumerate() {
                    let tile = tile.gid;
                    let pos = Vector2::new(32.0 * x as f32, 32.0 * y as f32);
                    println!("x: {} y: {} || px: {} py: {}", x, y, pos.x, pos.y);
                    gen.draw_texture_rec(
                        &floor_texture,
                        Rectangle::new((tile as f32 - 1.0) * 32.0, 0.0, 32.0, 32.0),
                        Vector2::new(32.0 * x as f32, 32.0 * y as f32),
                        Color::WHITE,
                    );
                }
            }
        }
    }
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RED);

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
