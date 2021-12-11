use bevy::{
    math::Vec2,
    prelude::{Assets, Commands, Res, ResMut},
    sprite2::Rect,
};
use kayak_core::render_primitive::RenderPrimitive;
use kayak_font::KayakFont;

use crate::{
    render::unified::pipeline::{ExtractQuadBundle, ExtractedQuad, UIQuadType},
    to_bevy_color, BevyContext,
};

use super::font_mapping::FontMapping;

pub fn extract_texts(
    mut commands: Commands,
    context: Res<BevyContext>,
    mut fonts: ResMut<Assets<KayakFont>>,
    font_mapping: Res<FontMapping>,
) {
    let render_commands = if let Ok(context) = context.kayak_context.read() {
        context.widget_manager.build_render_primitives()
    } else {
        vec![]
    };

    let text_commands: Vec<&RenderPrimitive> = render_commands
        .iter()
        .filter(|command| matches!(command, RenderPrimitive::Text { .. }))
        .collect::<Vec<_>>();

    let mut extracted_texts = Vec::new();
    for render_primitive in text_commands {
        let (background_color, layout, font_size, content, font) = match render_primitive {
            RenderPrimitive::Text {
                color,
                layout,
                size,
                content,
                font,
            } => (color, layout, *size, content, *font),
            _ => panic!(""),
        };

        let font_handle = font_mapping.get_handle(font).unwrap();
        let font = fonts.get(font_handle.clone()).unwrap();
        let max_glyph_size = font.sdf.max_glyph_size();

        let mut x = 0.0;
        for c in content.chars() {
            if let Some(glyph) = font.sdf.glyphs.iter().find(|glyph| glyph.unicode == c) {
                let plane_bounds = glyph.plane_bounds.as_ref();
                let (left, top, _width, _height) = match plane_bounds {
                    Some(val) => (
                        val.left,
                        val.top,
                        val.size().x * font_size,
                        val.size().y * font_size,
                    ),
                    None => (0.0, 0.0, 0.0, 0.0),
                };

                let font_ratio = font_size / font.sdf.atlas.size;
                let resized_max_glyph_size =
                    (max_glyph_size.x * font_ratio, max_glyph_size.y * font_ratio);

                let position_x = layout.posx + x + left * font_size;
                let position_y = (layout.posy + (-top * font_size)) + font_size;
                extracted_texts.push(ExtractQuadBundle {
                    extracted_quad: ExtractedQuad {
                        font_handle: Some(font_handle.clone()),
                        rect: Rect {
                            min: Vec2::new(position_x, position_y),
                            max: Vec2::new(
                                position_x + resized_max_glyph_size.0,
                                position_y + resized_max_glyph_size.1,
                            ),
                        },
                        color: to_bevy_color(background_color),
                        vertex_index: 0,
                        char_id: font.get_char_id(c).unwrap(),
                        z_index: layout.z_index,
                        quad_type: UIQuadType::Text,
                        type_index: 0,
                        border_radius: (0.0, 0.0, 0.0, 0.0),
                        image: None,
                        uv_max: None,
                        uv_min: None,
                    },
                });

                x += glyph.advance * font_size;
            }
        }
    }
    commands.spawn_batch(extracted_texts);
}
