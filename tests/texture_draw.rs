#[macro_use]
extern crate glium;

use glium::Surface;
use glium::GlObject;

mod support;

macro_rules! create_program {
    ($display:expr, $glsl_ty:expr, $glsl_value:expr) => (
        {
            let program = glium::Program::from_source(&$display,
                "
                    #version 110

                    attribute vec2 position;

                    void main() {
                        gl_Position = vec4(position, 0.0, 1.0);
                    }
                ",
                &format!("
                    #version 130

                    out {} color;

                    void main() {{
                        color = {};
                    }}
                ", $glsl_ty, $glsl_value),
                None);

            match program {
                Ok(p) => p,
                Err(_) => return
            }
        }
    );
}

macro_rules! draw_and_validate {
    ($display: expr, $program: expr, $texture:expr, $vb:expr, $ib:expr, $rust_value:expr) => (
        {
            $texture.as_surface().clear_color(0.0, 0.0, 0.0, 0.0);
            $texture.as_surface().draw(&$vb, &$ib, &$program, &uniform!{ texture: &$texture },
                                     &Default::default()).unwrap();

            $display.assert_no_error(None);

            let data: Vec<Vec<(u8, u8, u8, u8)>> = $texture.read();
            for row in data.iter() {
                for pixel in row.iter() {
                    assert_eq!(pixel, &$rust_value);
                }
            }

            $display.assert_no_error(None);
        }
    );
}

#[test]
fn texture_2d_draw() {
    let display = support::build_display();
    let (vb, ib) = support::build_rectangle_vb_ib(&display);

    let program = create_program!(display, "vec4", "vec4(1.0, 0.0, 1.0, 0.0)");

    let texture = glium::texture::Texture2d::empty(&display, 1024, 1024).unwrap();

    draw_and_validate!(display, program, texture, vb, ib, (255, 0, 255, 0));
}

#[test]
fn texture_2d_draw_unowned() {
    let display = support::build_display();
    let (vb, ib) = support::build_rectangle_vb_ib(&display);

    let program = create_program!(display, "vec4", "vec4(1.0, 0.0, 1.0, 0.0)");

    let empty_texture = glium::texture::Texture2d::empty_with_format(&display,
                                                                   glium::texture::UncompressedFloatFormat::F32F32F32F32,
                                                                   glium::texture::MipmapsOption::NoMipmap,
                                                                   1024, 1024).unwrap();
    let texture = unsafe {
        glium::texture::Texture2d::from_id(&display,
                                         glium::texture::UncompressedFloatFormat::F32F32F32F32,
                                         empty_texture.get_id(),
                                         false,
                                         glium::texture::MipmapsOption::NoMipmap,
                                         empty_texture.get_texture_type())
    };

    draw_and_validate!(display, program, texture, vb, ib, (255, 0, 255, 0));
}

