mod engine;
mod game;

extern crate piston_window;
extern crate find_folder;

use piston_window::*;
use crate::ActiveScreen::{MainGame, InitScreen, LooseScreen};
use piston_window::math::Scalar;
use winit::window::Icon;
use winit::platform::windows::IconExtWindows;

enum ActiveScreen {
    MainGame(game::MainGame),
    InitScreen,
    LooseScreen,
}

fn text_size<C>(
    font_size: types::FontSize,
    text: &str,
    cache: &mut C,
) -> Result<Size, C::Error>
    where
        C: character::CharacterCache,
{
    // TODO this assumes that text is ordered horizontal
    let mut width: Scalar = 0.0;
    let mut height: Scalar = 0.0;

    for ch in text.chars() {
        let character = cache.character(font_size, ch)?;

        width += character.advance_width();
        height = height.max(character.atlas_size[1] * 2.0 - character.top());
    }

    Ok(Size::from([width, height]))
}

fn draw_text_lines<C, G>(color: types::Color,
                         font_size: types::FontSize,
                         lines: &[&str],
                         area_size: Size,
                         cache: &mut C,
                         transform: math::Matrix2d,
                         g: &mut G,
) -> Result<(), C::Error>
    where
        C: character::CharacterCache,
        G: Graphics<Texture=<C as character::CharacterCache>::Texture>
{
    let line_spacing_add = font_size as f64 * 0.25;

    let mut sizes: Vec<Size> = Vec::new();
    let mut total_height = (lines.len() - 1) as f64 * line_spacing_add;

    for &line in lines {
        let s = text_size(font_size, line, cache)?;
        sizes.push(s);
        total_height += s.height;
    }

    let mut y_pos = (area_size.height - total_height) / 2.0 + sizes[0].height;

    for (index, line) in lines.iter().enumerate() {
        let s = sizes[index];

        let x_pos = (area_size.width - s.width) / 2.0;

        text(color, font_size, line, cache,
             transform.trans(x_pos, y_pos), g)?;

        y_pos += s.height + line_spacing_add;
    }

    Ok(())
}

fn draw_text_multiline<C, G>(color: types::Color,
                             font_size: types::FontSize,
                             text: &str,
                             area_size: Size,
                             cache: &mut C,
                             transform: math::Matrix2d,
                             g: &mut G,
) -> Result<(), C::Error>
    where
        C: character::CharacterCache,
        G: Graphics<Texture=<C as character::CharacterCache>::Texture>
{
    draw_text_lines(color, font_size, &text.lines().collect::<Vec<&str>>(), area_size, cache, transform, g)
}

fn main() {
    let rect_size = 20.0;
    let field_size = 20.0;

    let mut window: PistonWindow =
        WindowSettings::new("Rust Snake", [rect_size * field_size; 2])
            .build().unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    println!("Found assets: {:?}", assets);
    let mut glyphs = window.load_font(assets.join("FiraSans-Regular.ttf")).unwrap();


    let mut active = InitScreen;

    let icon = Icon::from_path(assets.join("icon.ico"), None);
    println!("icon: {:?}", icon);
    window.window. ctx.window().set_window_icon(icon.ok());

    while let Some(e) = window.next() {
        match &mut active {
            MainGame(ref mut game) => {
                if let Some(button) = e.press_args() {
                    match button {
                        Button::Keyboard(Key::Escape) => {
                            active = InitScreen;
                            continue;
                        }
                        _ => (),
                    }
                }

                if !game.run(&mut window, &mut glyphs, e) {
                    active = LooseScreen;
                }
            }
            InitScreen => {
                if let Some(button) = e.press_args() {
                    match button {
                        Button::Keyboard(Key::Return) => {
                            active = MainGame(game::MainGame::new(rect_size, field_size));
                        }
                        Button::Keyboard(Key::Escape) => {
                            return;
                        }
                        _ => (),
                    }
                }

                window.draw_2d(&e, |c, g, device| {
                    clear([0.95, 0.95, 0.95, 1.0], g);

                    draw_text_multiline([0.0, 0.0, 0.0, 1.0], 64, "Snake Game", Size::from([rect_size * field_size, 100.0]),
                                        &mut glyphs, c.transform, g,
                    ).unwrap();

                    draw_text_multiline([0.0, 0.8, 0.0, 1.0], 48, "Press enter\nto start!", Size::from([rect_size * field_size, rect_size * field_size - 100.0]),
                                        &mut glyphs, c.transform.trans(0.0, 100.0), g,
                    ).unwrap();


                    // Update glyphs before rendering.
                    glyphs.factory.encoder.flush(device);
                });
            }
            LooseScreen => {
                if let Some(button) = e.press_args() {
                    match button {
                        Button::Keyboard(Key::Return) => {
                            active = InitScreen;
                        }
                        Button::Keyboard(Key::Escape) => {
                            return;
                        }
                        _ => (),
                    }
                }

                window.draw_2d(&e, |c, g, device| {
                    clear([0.95, 0.95, 0.95, 1.0], g);


                    draw_text_multiline([0.8, 0.0, 0.0, 1.0], 48, "You lost!\n \nPress enter\nto return to\nmain screen.",
                                        Size::from([rect_size * field_size, rect_size * field_size]),
                                        &mut glyphs, c.transform, g,
                    ).unwrap();


                    // Update glyphs before rendering.
                    glyphs.factory.encoder.flush(device);
                });
            }
        }
    }
}
