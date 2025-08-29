use cube_core::utils::cube_utils::Color;

use crate::game::Game;

pub fn draw(game: &mut Game) {
    for (i, pixel) in game.pixels.frame_mut().chunks_exact_mut(4).enumerate() {
        let x = (i % game.args.width as usize) as u32;
        let y = (i / game.args.width as usize) as u32;

        let rgba = if let Some(color) = game.screen.color_at(x as i16, y as i16) {
            color.rgba()
        } else {
            Color::Black.rgba()
        };

        pixel.copy_from_slice(&rgba);
    }

    draw_time(game);
}

fn draw_time(game: &mut Game) {
    if let Some(timer) = game.timer.as_mut() {
        let elapsed = timer.update_elapsed();

        let minutes = (elapsed / 60.0).floor() as u32;
        let seconds = elapsed % 60.0;

        let time = if minutes > 0 {
            format!("{}:{:.2}", minutes, seconds)
        } else {
            format!("{:.2}", seconds)
        };
        
        draw_text(
            game,
            time.as_str(),
            10,
            game.args.height as i32 - 30,
            30.0,
        );
    }
}

fn draw_text(
    game: &mut Game,
    text: &str,
    x: i32,
    y: i32,
    scale_px: f32,
) {
    use rusttype::{Scale, point};

    let frame = game.pixels.frame_mut();
    let font = &game.font;
    let scale = Scale::uniform(scale_px);
    let v_metrics = font.v_metrics(scale);
    let glyphs: Vec<_> = font
        .layout(text, scale, point(x as f32, y as f32 + v_metrics.ascent))
        .collect();

    let background_rgba = Color::Black.rgba();
    for glyph in glyphs {
        if let Some(bb) = glyph.pixel_bounding_box() {
            glyph.draw(|gx, gy, v| {
                let px = gx as i32 + bb.min.x;
                let py = gy as i32 + bb.min.y;
                if px >= 0 && py >= 0 && (px as u32) < game.args.width && (py as u32) < game.args.height {
                    let idx = ((py as u32 * game.args.width + px as u32) * 4) as usize;
                    let intensity = (v * 255.0) as u8;
                    frame[idx] = intensity.saturating_add(background_rgba[0]);         // R
                    frame[idx + 1] = intensity.saturating_add(background_rgba[1]);     // G
                    frame[idx + 2] = intensity.saturating_add(background_rgba[2]);     // B
                    frame[idx + 3] = 255;                                              // A
                }
            });
        }
    }
}