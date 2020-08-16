//#![windows_subsystem = "windows"]

mod game;
mod numbers;

static FONT: &str = "BebasNeue-Regular.ttf";


fn main() {

    let mut game = game::Game::new(
        "snake", 
        20,

        50, 
        30, 

        66, 
        600, 
        16
    ).unwrap();
    
    game.ptr = Some(&mut game);

    loop {
        
        let tm = std::time::Instant::now();
        
        game.recv_input();
        game.process();
        game.render();


        let tms = tm.elapsed().as_millis() as u32;

        if tms < game.mps {
            #[allow(deprecated)]
            std::thread::sleep_ms(game.mps - tms);
        }

    }
}