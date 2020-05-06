//use raylib::core::color::Color;

use raylib::ffi::*;
fn main() {
    unsafe {
        let screenWidth = 800;
        let screenHeight = 450;
        let tile_height = 16;
        let tile_width = 20;
        InitWindow(
            screenWidth,
            screenHeight,
            "raylib template - simple game".as_ptr() as *const i8,
        );
        SetTargetFPS(30); // Set desired framerate (frames-per-second)

        let draw_buffer = LoadRenderTexture(640, 480);
        //SetTextureFilter((RenderTexture2D)draw_buffer, FILTER_POlet);

        BeginDrawing();
        BeginTextureMode(draw_buffer);
        for y in 0..tile_height {
            for x in 0..tile_width {
                // dbg!(tile_id);

                let pos = Vector2 {
                    x: (32 * x) as f32,
                    y: (32 * y) as f32,
                };
                DrawRectangle(
                    pos.x as i32,
                    pos.y as i32,
                    32,
                    32,
                    Color {
                        r: 255,
                        g: 0,
                        b: 0,
                        a: 155,
                    },
                );
            }
        }
        EndTextureMode();
        EndDrawing();

        // Main game loop
        while (!WindowShouldClose())
        // Detect window close button or ESC key
        {
            BeginDrawing();
            ClearBackground(Color {
                r: 0,
                g: 0,
                b: 0,
                a: 255,
            });

            let src = Rectangle {
                x: 0.0,
                y: 0.0,
                width: draw_buffer.texture.width as f32,
                height: -draw_buffer.texture.height as f32,
            };
            DrawTextureRec(
                draw_buffer.texture,
                src,
                Vector2 { x: 0.0, y: 0.0 },
                Color {
                    r: 255,
                    g: 255,
                    b: 255,
                    a: 255,
                },
            );
            EndDrawing();
        }

        // De-Initialization
        //--------------------------------------------------------------------------------------

        // TODO: Unload all loaded data (textures, fonts, audio) here!

        CloseWindow(); // Close window and OpenGL context
                       //--------------------------------------------------------------------------------------
    }
}
