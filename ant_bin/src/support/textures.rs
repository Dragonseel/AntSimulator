use std::io::Cursor;

use glium::{Display, Texture2d};

pub struct TextureContainer {
    pub ground_texture: Texture2d,
    pub ant_texture: Texture2d,
    pub food_texture: Texture2d,
    pub nest_texture: Texture2d,
}

impl TextureContainer {
    pub fn new(display: &Display) -> TextureContainer {
        // Load the texture.
        let ground_texture = {
            let img = image::load(
                Cursor::new(&include_bytes!("../../../assets/ground.png")[..]),
                image::ImageFormat::Png,
            )
            .unwrap()
            .to_rgba32f();

            let img_dim = img.dimensions();
            let img = glium::texture::RawImage2d::from_raw_rgba_reversed(&img.into_raw(), img_dim);

            glium::texture::Texture2d::new(display, img).unwrap()
        };

        // Load the texture.
        let ant_texture = {
            let img = image::load(
                Cursor::new(&include_bytes!("../../../assets/ant.png")[..]),
                image::ImageFormat::Png,
            )
            .unwrap()
            .to_rgba32f();

            let img_dim = img.dimensions();
            let img = glium::texture::RawImage2d::from_raw_rgba_reversed(&img.into_raw(), img_dim);

            glium::texture::Texture2d::new(display, img).unwrap()
        };

        // Load the texture.
        let food_texture = {
            let img = image::load(
                Cursor::new(&include_bytes!("../../../assets/food.png")[..]),
                image::ImageFormat::Png,
            )
            .unwrap()
            .to_rgba32f();

            let img_dim = img.dimensions();
            let img = glium::texture::RawImage2d::from_raw_rgba_reversed(&img.into_raw(), img_dim);

            glium::texture::Texture2d::new(display, img).unwrap()
        };

        // Load the texture.
        let nest_texture = {
            let img = image::load(
                Cursor::new(&include_bytes!("../../../assets/nest.png")[..]),
                image::ImageFormat::Png,
            )
            .unwrap()
            .to_rgba32f();

            let img_dim = img.dimensions();
            let img = glium::texture::RawImage2d::from_raw_rgba_reversed(&img.into_raw(), img_dim);

            glium::texture::Texture2d::new(display, img).unwrap()
        };

        TextureContainer {
            ground_texture,
            ant_texture,
            food_texture,
            nest_texture,
        }
    }
}
