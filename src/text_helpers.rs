use piston_window::math::Scalar;
use piston_window::*;

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

        piston_window::text(color, font_size, line, cache,
             transform.trans(x_pos, y_pos), g)?;

        y_pos += s.height + line_spacing_add;
    }

    Ok(())
}

pub fn draw_text_multiline<C, G>(color: types::Color,
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
