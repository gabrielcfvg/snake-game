use sdl2::render::TextureCreator;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::WindowContext;
use sdl2::pixels::Color;

pub fn initialize_numbers(texture_creator: TextureCreator<WindowContext>, ttf_context: &Sdl2TtfContext) -> Result<Vec<sdl2::render::Texture>, Box<dyn std::error::Error>> {

    let font = ttf_context.load_font(crate::FONT, 50)?;
    let mut saida = vec![];

    for i in 0..10 {
        let tmp = font.render(&i.to_string()).blended(Color::RGB(255, 255, 255))?;
        saida.push(texture_creator.create_texture_from_surface(&tmp)?);
    }
    Ok(saida)
}
