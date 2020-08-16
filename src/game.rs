use sdl2;
use sdl2::image::LoadTexture;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use rand::Rng;
use sdl2::pixels::Color;

#[derive(PartialEq, Copy, Clone)]
enum Direction {
    Above,
    Below,
    Left,
    Right
}


pub struct Game {

    pub ptr: Option<*mut Game>,

    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    event_pump: sdl2::EventPump,
    rnd: rand::prelude::ThreadRng,
    snake_tex: (sdl2::render::Texture, sdl2::render::Texture),

    cell_size: i32,
    cell_x_count: i32,
    cell_y_count: i32,

    snake: Vec<(i32, i32)>,
    direction: Direction,
    current_direction: Direction,

    apple_pos: Option<(i32, i32)>,
    apple_tex: sdl2::render::Texture,

    text_texture: sdl2::render::Texture,
    numbers: Vec<sdl2::render::Texture>,
    deaths_tex: sdl2::render::Texture,

    slow_mps: u32,
    normal_mps: u32,
    fast_mps: u32,
    pub mps: u32,
    slow: bool,

    points: u32,
    deaths: u32


}


impl Game {

    pub fn new(name: &str, cell_size: u32, cell_x_count: u32, cell_y_count: u32, normal_mode: u32, slow_mode: u32, fast_mode: u32) -> Result<Self, Box<dyn std::error::Error>> {

        let sdl_context = sdl2::init()?;
        let video_system = sdl_context.video()?;
        let window = video_system.window(name, cell_x_count * cell_size, cell_y_count * cell_size).opengl().build()?;
        let canvas = window.into_canvas().build()?;
        let event_pump = sdl_context.event_pump()?;
        let texture_creator = canvas.texture_creator();

        let head_tex = texture_creator.load_texture("sprites/cabeça.png")?;
        let body_tex = texture_creator.load_texture("sprites/corpo.png")?;
        let apple_tex = texture_creator.load_texture("sprites/maça.png")?;

        let ttf_context = sdl2::ttf::init()?;
        let font = ttf_context.load_font(crate::FONT, 50)?;
        let tmp = font.render("POINTS: ").blended(Color::RGB(255, 255, 255))?;
        let points = texture_creator.create_texture_from_surface(&tmp)?;
        
        let tmp = font.render("DEATHS: ").blended(Color::RGB(255, 255, 255))?;
        let death = texture_creator.create_texture_from_surface(&tmp)?;
        

        let mut game = Game {

                ptr: None,

                canvas,
                event_pump,
                rnd: rand::thread_rng(),
                snake_tex: (body_tex, head_tex),

                cell_size: cell_size as i32,
                cell_x_count: cell_x_count as i32,
                cell_y_count: cell_y_count as i32,

                snake: vec![(2, 5), (1, 5)],
                direction: Direction::Right,
                current_direction: Direction::Right,

                apple_pos: Some(((cell_x_count-1) as i32, (cell_y_count-1) as i32)),
                apple_tex,

                text_texture: points,
                numbers: crate::numbers::initialize_numbers(texture_creator, &ttf_context)?,
                deaths_tex: death,

                slow_mps: slow_mode,
                normal_mps: normal_mode,
                fast_mps: fast_mode,
                mps: normal_mode,
                slow: false,

                points: 0,
                deaths: 0
            };
        
        game.gen_apple();

        Ok(game)
    }


    pub fn recv_input(&mut self) {

        let mut cheat_num: u32 = 0;

        for event in self.event_pump.poll_iter() {
            match event {
                
                Event::Quit{timestamp: _} => {
                    std::process::exit(0);
                }

                Event::KeyDown{timestamp: _, window_id: _, keycode: Some(Keycode::Backspace), scancode: _, keymod: _, repeat: _} => {
                    cheat_num += 3;
                    self.points += 3;
                }
        
                Event::KeyDown{timestamp: _, window_id: _, keycode: Some(Keycode::Space), scancode: _, keymod: _, repeat: _} => {
                    if self.slow {
                        self.mps = self.normal_mps;
                        self.slow = false;
                    }
                    else {
                        self.mps = self.slow_mps;
                        self.slow = true;
                    }
                }

                Event::KeyDown{timestamp: _, window_id: _, keycode: Some(Keycode::F), scancode: _, keymod: _, repeat: _} => {
                    if self.mps == self.normal_mps {
                        self.mps = self.fast_mps;
                    }
                }
                Event::KeyUp{timestamp: _, window_id: _, keycode: Some(Keycode::F), scancode: _, keymod: _, repeat: _} => {
                    if self.mps == self.fast_mps {
                        self.mps = self.normal_mps;
                    }
                }

                Event::KeyDown{timestamp: _, window_id: _, keycode: Some(Keycode::Right), scancode: _, keymod: _, repeat: _} => {

                    if (self.direction == Direction::Above || self.direction == Direction::Below) &&
                       (self.current_direction == Direction::Above || self.current_direction == Direction::Below)
                    {
                        self.direction = Direction::Right
                    }
                       
                }
                Event::KeyDown{timestamp: _, window_id: _, keycode: Some(Keycode::Left), scancode: _, keymod: _, repeat: _} => {
                    
                    if (self.direction == Direction::Above || self.direction == Direction::Below) &&
                       (self.current_direction == Direction::Above || self.current_direction == Direction::Below) 
                    {
                        self.direction = Direction::Left
                    }
                }
                Event::KeyDown{timestamp: _, window_id: _, keycode: Some(Keycode::Up), scancode: _, keymod: _, repeat: _} => {
                    
                    if (self.direction == Direction::Left || self.direction == Direction::Right) &&
                       (self.current_direction == Direction::Left || self.current_direction == Direction::Right)
                    {
                        self.direction = Direction::Above;
                    }
                }
                Event::KeyDown{timestamp: _, window_id: _, keycode: Some(Keycode::Down), scancode: _, keymod: _, repeat: _} => {
                    
                    if (self.direction == Direction::Left || self.direction == Direction::Right) &&
                       (self.current_direction == Direction::Left || self.current_direction == Direction::Right) 
                    {
                        self.direction = Direction::Below;
                    }
                }
                _ => ()
            }
        }
        for _ in 0..cheat_num {
            let next: (i32, i32) = {
                match self.direction {
                    Direction::Above => {
                        (self.snake[0].0, (self.snake[0].1) - 1)
                    }
                    Direction::Below => {
                        (self.snake[0].0, (self.snake[0].1) + 1)
                    }
                    Direction::Left => {
                        ((self.snake[0].0) - 1, self.snake[0].1)
                    }
                    Direction::Right => {
                        ((self.snake[0].0) + 1, self.snake[0].1)
                    }
                }
            };
            
            self.replace_snake(next, true);
        }

    }


    pub fn process(&mut self) {

        let next: (i32, i32) = {
            match self.direction {
                Direction::Above => {
                    (self.snake[0].0, (self.snake[0].1) - 1)
                }
                Direction::Below => {
                    (self.snake[0].0, (self.snake[0].1) + 1)
                }
                Direction::Left => {
                    ((self.snake[0].0) - 1, self.snake[0].1)
                }
                Direction::Right => {
                    ((self.snake[0].0) + 1, self.snake[0].1)
                }
            }
        };

        self.current_direction = self.direction;

        // caso a cobra entre em contato com ela mesma
        if self.snake.contains(&next) {
            
            #[allow(deprecated)]
            std::thread::sleep_ms(500);
            
            self.restart();
            self.points = 0;
            self.deaths += 1;
            return;
            //panic!("game over");
        }


        if next == self.apple_pos.unwrap() {
            self.points += 1;
            self.replace_snake(next, true);
            self.gen_apple();
        }
        else {
            self.replace_snake(next, false);
        }

    }


    pub fn render(&mut self) {

        self.canvas.clear();

        // rendenização da cabeça
        self.canvas.copy(&self.snake_tex.1, None, Rect::new(self.cell_size * self.snake[0].0, self.cell_size * self.snake[0].1, self.cell_size as u32, self.cell_size as u32)).unwrap();

        // rendenização do corpo
        for part in self.snake.iter().skip(1) {
            self.canvas.copy(&self.snake_tex.0,
                             None, 
                             Rect::new(self.cell_size * part.0, 
                                self.cell_size * part.1, 
                                self.cell_size as u32, 
                                self.cell_size as u32)
                            ).unwrap();
        }
        
        //rendenização da maça
        self.canvas.copy(&self.apple_tex, 
                         None, 
                         Rect::new(self.cell_size * self.apple_pos.unwrap().0, 
                                self.cell_size * self.apple_pos.unwrap().1,
                                self.cell_size as u32, self.cell_size as u32)
                            ).unwrap();

        self.canvas.copy(&self.text_texture, 
            None, 
            Rect::new(
                0, 
                0, 
                (self.cell_size*6) as u32, 
                (self.cell_size*2) as u32)).unwrap();
        
        for (i, ch) in self.points.to_string().chars().enumerate() {

            let texture = &self.numbers[ch.to_digit(10).unwrap() as usize];

            self.canvas.copy(
                texture, 
                None, 
                Rect::new(
                    ((i + 6) * self.cell_size as usize) as i32, 
                    0, 
                    ((self.cell_size*2) as f64 * 0.5) as u32, 
                    (self.cell_size*2) as u32)).unwrap();
        }      

        let padd = self.points.to_string().len();

        self.canvas.copy(&self.deaths_tex, 
            None, 
            Rect::new(
                ((8 + padd) * self.cell_size as usize) as i32, 
                0, 
                (self.cell_size*6) as u32, 
                (self.cell_size*2) as u32)).unwrap();

        for (i, ch) in self.deaths.to_string().chars().enumerate() {
            
            let texture = &self.numbers[ch.to_digit(10).unwrap() as usize];

            self.canvas.copy(
                texture, 
                None, 
                Rect::new(
                    ((i + 15 + padd) * self.cell_size as usize) as i32, 
                    0, 
                    ((self.cell_size*2) as f64 * 0.5) as u32, 
                    (self.cell_size*2) as u32)).unwrap();
        }

        self.canvas.present();

        
    }


    fn gen_apple(&mut self) {

        loop {

            let next: (i32, i32) = (self.rnd.gen_range(0, self.cell_x_count), self.rnd.gen_range(0, self.cell_y_count));

            if !self.snake.contains(&next) {
                self.apple_pos = Some(next);
                break;
            }
        }
    }


    pub fn restart(&mut self) {
        self.direction = Direction::Right;
        self.snake.truncate(2);
        self.snake = vec![(2, 5), (1, 5)];
        self.gen_apple();
    }


    fn replace_snake(& self, mut next: (i32, i32), add: bool) {       

        unsafe {

            if !(
                next.0 >= 0 && next.0 < self.cell_x_count &&
                next.1 >= 0 && next.1 < self.cell_y_count
                )
            
            {
                if next.0 < 0 {
                    next.0 = self.cell_x_count-1;
                }
                else if next.0 >= self.cell_x_count {
                    next.0 = 0;
                }
                
                if next.1 < 0 {
                    next.1 = self.cell_y_count-1;
                }
                else if next.1 >= self.cell_y_count {
                    next.1 = 0
                }
                
            }
            
            let mut tmp: (i32, i32);
            let mut troca: (i32, i32);

            tmp = self.snake[0];
            (*self.ptr.unwrap()).snake[0] = next;

            for part in (*self.ptr.unwrap()).snake.iter_mut().skip(1) {

                troca = (part.0, part.1);
                (*part).0 = tmp.0;
                (*part).1 = tmp.1;
                tmp = troca;
            }

            if add {
                (*self.ptr.unwrap()).snake.push(tmp);
            }
        }

    }
}
