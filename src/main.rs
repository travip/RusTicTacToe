extern crate piston_window;
extern crate find_folder;
extern crate image;

use piston_window::*;


#[derive(Copy, Clone, PartialEq, Eq)]
enum Token {
    Cross,
    Circle,
    Empty,
}

struct GridSquare {
    token : Token,
    coords : [f64; 4],
}

struct GameBoard {
    grid : Vec<GridSquare>,
    vert1 : [f64; 4],
    vert2 : [f64; 4],
    horz1 : [f64; 4],
    horz2 : [f64; 4]
}

trait GetSquare {
    fn getSquare(&self, x : f64, y : f64) -> usize;
}

trait ResetGameBoard {
    fn reset(&mut self) -> ();
}

impl GetSquare for GameBoard {
    fn getSquare(&self, x : f64, y : f64) -> usize {
        for (i, square) in self.grid.iter().enumerate() {
            if (x > square.coords[0]) && (x < square.coords[2]) && (y > square.coords[1]) && (y < square.coords[3]) {
                return i as usize;
            }
        }
        0 as usize
    }
}

impl ResetGameBoard for GameBoard {
    fn reset(&mut self) -> () {
        for i in 0..9 {
            self.grid[i].token = Token::Empty;
        }
    }
}

// Check for win, return a token indicating winning player (or Empty on no win)
fn CheckForWin(grid : &Vec<GridSquare>) -> Token {

    // Construct the magic square
    let mut win_grid =  vec![8, 1, 6, 3, 5, 7, 4, 9, 2];
    for (i, square) in grid.iter().enumerate() {
        match square.token {
            Token::Cross => win_grid[i] *= 2,
            Token::Circle => win_grid[i] *= -2,
            _ => win_grid[i] += 0
        }
    }
    // Check rows
    for i in 0..3 {
        let sum = win_grid[i*3] + win_grid[i*3 + 1] + win_grid[i*3 + 2];
        if sum == 30 {
            println!("Crosses won!");
            return Token::Cross
        }
        else if sum == -30 {
            println!("Cirlces won!");
            return Token::Circle;
        }
    }

    // Check columns
    for i in 0..3 {
        let sum = win_grid[i] + win_grid[i + 3] + win_grid[i + 6];
        if sum == 30 {
            println!("Crosses won!");
            return Token::Cross
        }
        else if sum == -30 {
            println!("Cirlces won!");
            return Token::Circle;
        }
    }

    // Check Diagonals
    let sum = win_grid[0] + win_grid[4] + win_grid[8];
    if sum == 30 {
        println!("Crosses won!");
        return Token::Cross
    }
    else if sum == -30 {
        println!("Cirlces won!");
        return Token::Circle;
    }

    let sum = win_grid[2] + win_grid[4] + win_grid[6];
    if sum == 30 {
        println!("Crosses won!");
        return Token::Cross
    }
    else if sum == -30 {
        println!("Cirlces won!");
        return Token::Circle;
    }

    Token::Empty
}

fn main() {

    // Open window
    let mut window: PistonWindow = WindowSettings::new(
            "Tic Tac Toe",
            [600, 600]
        )
        .exit_on_esc(true)
        .build()
        .unwrap();

    let assets = find_folder::Search::ParentsThenKids(3,3)
        .for_folder("assets").unwrap();

    // Cross image
    let cross = Texture::from_path(
        &mut window.factory,
        assets.join("cross.png"),
        Flip::None,
        &TextureSettings::new()
    ).unwrap();

    // Circle image
    let circle = Texture::from_path(
        &mut window.factory,
        assets.join("circle.png"),
        Flip::None,
        &TextureSettings::new()
    ).unwrap();

    // Initialize mouse
    let mut mouse_pos = (-1.0 as f64, -1.0 as f64);
    let mut cross_turn = true;
    let mut cross_score = 0;
    let mut circle_score = 0;

    // Initiaize game board
    let mut game_board = GameBoard { grid : Vec::<GridSquare>::new(),
                                     vert1 : [195.0, 0.0, 10.0, 595.0],
                                     vert2 : [395.0, 00.0, 10.0, 595.0],
                                     horz1 : [5.0, 195.0, 595.0, 10.0],
                                     horz2 :[5.0, 395.0, 595.0, 10.0],
                                   };
    for i in 0..9 {
        game_board.grid.push(GridSquare {token : Token::Empty,
                                    coords : [(i % 3 * 200 + 5) as f64, 
                                              (i / 3 * 200 + 5) as f64, 
                                              (((i % 3) + 1) * 200 - 5) as f64, 
                                              (((i / 3) + 1) * 200 - 5) as f64]
                        });
                        
    }

    window.set_lazy(true);
    while let Some(e) = window.next() {
        // Draw window
        window.draw_2d(&e, |c, g| {
            clear([255.0, 255.0, 255.0, 1.0], g);
            // Vertical 
            rectangle([0.0, 0.0, 0.0, 1.0],
                     game_board.vert1,
                     c.transform, g);
            rectangle([0.0, 0.0, 0.0, 1.0],
                     game_board.vert2,
                     c.transform, g);
            // Horizontal
            rectangle([0.0, 0.0, 0.0, 1.0],
                     game_board.horz1,
                     c.transform, g);
            rectangle([0.0, 0.0, 0.0, 1.0],
                     game_board.horz2,
                     c.transform, g);
            
            for square in &game_board.grid{
                match square.token {
                    Token::Cross => image(&cross,
                                        c.transform.trans(square.coords[0], square.coords[1]), g),
                    Token::Circle => image(&circle,
                                        c.transform.trans(square.coords[0], square.coords[1]), g),
                    _ => rectangle([0.0, 0.0, 0.0, 0.0],
                                   [square.coords[0], square.coords[1], 190.0, 175.0],
                                   c.transform, g),
                }
            }

        });

        // Track mouse position
        if let Some(pos) = e.mouse_cursor_args(){
            mouse_pos = (pos[0] as f64, pos[1] as f64);
        }

        // On left click
        if let Some(button) = e.press_args() {
            use piston_window::Button::Mouse;
            if button == Mouse(MouseButton::Left) {
                let sq = game_board.getSquare(mouse_pos.0, mouse_pos.1);
                // If place is empty, put token
                if game_board.grid[sq].token == Token::Empty {
                    if cross_turn {game_board.grid[sq].token = Token::Cross;}
                    else {game_board.grid[sq].token = Token::Circle}
                    cross_turn = !cross_turn;
                    // Check for win
                    match CheckForWin(&game_board.grid) {
                        Token::Cross => {cross_score += 1; game_board.reset();},
                        Token::Circle => {circle_score += 1; game_board.reset();},
                        _ => ()
                    
                    }
                }
            }
        }
    }
}