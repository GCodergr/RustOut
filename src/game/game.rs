extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::TextureCreator;
use sdl2::image::LoadTexture;
use sdl2::render::TextureQuery;

use std::time::Duration;

use crate::game::physics::{check_for_collisions, resolve_ball_wall_collisions, resolve_ball_paddle_collisions, resolve_ball_brick_collisions};
use crate::game::font_utilities::{get_top_right_rect, get_top_left_rect};

pub const WINDOW_WIDTH: u32 = 800;
pub const WINDOW_HEIGHT: u32 = 600;

const STARTING_PLAYER_LIVES: i32 = 3;

const BALL_START_POSITION_X: i32 = 50;
const BALL_START_POSITION_Y: i32 = 450;

const BALL_START_SPEED_X: f32 = 150.0;
const BALL_START_SPEED_Y: f32 = -150.0;

const BRICK_ROW_COUNT: u32 = 4;
const BRICK_COLUMN_COUNT: u32 = 8;

#[derive(Copy, Clone)]
pub struct Brick {
    pub rect: Rect,
    pub active: bool,
}

// Default Trait for Brick
impl Default for Brick {
    fn default() -> Brick {
        Brick {
            rect: Rect::new(0, 0, 0, 0),
            active: true, // Bricks are active by default
        }
    }
}

#[derive(Copy, Clone)]
pub struct Vector2 {
    pub x : f32,
    pub y : f32,
}

#[derive(PartialEq, Eq)]
enum GameplayState {
    Normal,
    BallFrozen,
}

pub fn run() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let tff_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

    let window = video_subsystem.window("RustOut", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas()
        .accelerated()
        .build()
        .map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(100, 149, 237));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().map_err(|e| e.to_string())?;

    let texture_creator: TextureCreator<_> = canvas.texture_creator();

    let paddle_texture = texture_creator.load_texture("assets/textures/paddle_04.png")?;
    let ball_texture = texture_creator.load_texture("assets/textures/ballGrey_01.png")?;
    let brick_texture = texture_creator.load_texture("assets/textures/tileYellow_02.png")?;

    let font = tff_context.load_font("assets/fonts/Roboto-Regular.ttf", 24)?;

    let mut player_lives = STARTING_PLAYER_LIVES;

    let mut paddle_rect = Rect::new(50, 530, 62, 12);
    let mut paddle_position = Vector2 { x: 50.0, y : 530.0};
    let paddle_speed_x : f32  = 600.0;

    let mut ball_rect = Rect::new(BALL_START_POSITION_X, BALL_START_POSITION_Y, 8, 8);
    let mut ball_position = Vector2 { x: BALL_START_POSITION_X as f32, y : BALL_START_POSITION_Y as f32};
    let mut ball_speed = Vector2 {x: BALL_START_SPEED_X, y: BALL_START_SPEED_Y};

    let mut bricks: [[Brick; BRICK_COLUMN_COUNT as usize]; BRICK_ROW_COUNT as usize] =
        [[Brick::default(); BRICK_COLUMN_COUNT as usize]; BRICK_ROW_COUNT as usize]; // Brick is initialized with default values
    let mut brick_count: i32 = (BRICK_ROW_COUNT * BRICK_COLUMN_COUNT) as i32;

    const BRICK_BOARD_START_X: usize = 150;
    const BRICK_BOARD_START_Y: usize = 50;

    const BRICK_OFFSET_X: usize = 28;
    const BRICK_OFFSET_Y: usize = 12;

    const BRICK_WIDTH: usize = 40;
    const BRICK_HEIGHT: usize = 16;

    for row in 0..BRICK_ROW_COUNT {
        for column in 0..BRICK_COLUMN_COUNT {
            bricks[row as usize][column as usize].rect = Rect::new((BRICK_BOARD_START_X + (column as usize * (BRICK_WIDTH + BRICK_OFFSET_X))) as i32,
                                                                   (BRICK_BOARD_START_Y + (row as usize * (BRICK_HEIGHT + BRICK_OFFSET_Y))) as i32,
                                                                   BRICK_WIDTH as u32,
                                                                   BRICK_HEIGHT as u32);

            bricks[row as usize][column as usize].active = true;
        }
    }

    let lives_label_text = "Lives: ";
    let score_label_text = "Score: ";
    let mut bricks_destroyed: i32 = 0;
    const SCORE_MULTIPLIER: i32 = 10;

    let mut current_game_state = GameplayState::Normal;

    let mut previous_time: u32;
    let mut current_time: u32 = 0;
    let mut delta_time: f32; // Time between the current and previous frame in seconds
    let mut timer = sdl_context.timer()?;

    'running: loop {
        previous_time = current_time;
        current_time = timer.ticks();
        delta_time = (current_time - previous_time) as f32 / 1000.0;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    paddle_position.x -= paddle_speed_x * delta_time;
                    paddle_rect.set_x(paddle_position.x as i32);
                    check_for_collisions(&mut paddle_rect);
                }
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    paddle_position.x += paddle_speed_x * delta_time;
                    paddle_rect.set_x(paddle_position.x as i32);
                    check_for_collisions(&mut paddle_rect);
                }
                _ => {}
            }
        }

        //
        // Update Logic
        //

        if current_game_state == GameplayState::Normal {
            update_ball_position(&mut ball_rect, ball_speed, &mut ball_position, delta_time);
            resolve_ball_wall_collisions(&mut ball_rect, &mut ball_speed, &mut player_lives);
            resolve_ball_paddle_collisions(ball_rect, &mut ball_speed, paddle_rect);

            for row in 0..BRICK_ROW_COUNT {
                for column in 0..BRICK_COLUMN_COUNT {
                    resolve_ball_brick_collisions(ball_rect, &mut ball_speed, &mut bricks[row as usize][column as usize],
                                                  &mut brick_count, &mut bricks_destroyed);
                }
            }

            if check_for_victory_conditions(player_lives, brick_count) {
                restart_game(&mut ball_rect, &mut ball_speed, &mut player_lives, &mut bricks_destroyed);

                for row in 0..BRICK_ROW_COUNT {
                    for column in 0..BRICK_COLUMN_COUNT {
                        bricks[row as usize][column as usize].active = true;
                    }
                }

                brick_count = (BRICK_ROW_COUNT * BRICK_COLUMN_COUNT) as i32;
            }
        }

        //
        // Update Rendering
        //

        // We fill our window with a background color
        canvas.set_draw_color(Color::RGB(41, 41, 41));
        // We draw it
        canvas.clear();

        canvas.copy(&paddle_texture,
                    None,
                    paddle_rect)
            .expect("Couldn't copy texture into windows");

        canvas.copy(&ball_texture,
                    None,
                    ball_rect)
            .expect("Couldn't copy texture into windows");

        for row in 0..BRICK_ROW_COUNT {
            for column in 0..BRICK_COLUMN_COUNT {
                if bricks[row as usize][column as usize].active {
                    canvas.copy(&brick_texture,
                                None,
                                bricks[row as usize][column as usize].rect)
                        .expect("Couldn't copy texture into windows");
                }
            }
        }

        let lives_text = lives_label_text.to_owned() + &player_lives.to_string();

        let lives_text_surface = font.render(&lives_text).blended(Color::RGBA(100, 149, 237, 255))
            .map_err(|e| e.to_string())?;
        let lives_font_texture = texture_creator.create_texture_from_surface(&lives_text_surface)
            .map_err(|e| e.to_string())?;

        let TextureQuery { width: lives_text_width, height: lives_text_height, .. } = lives_font_texture.query();

        let lives_text_padding = 64;
        let lives_text_target = get_top_right_rect(lives_text_width,
                                                   lives_text_height,
                                                   WINDOW_WIDTH - lives_text_padding,
                                                   WINDOW_HEIGHT - lives_text_padding);

        canvas.copy(&lives_font_texture, None, Some(lives_text_target))?;

        let score = bricks_destroyed * SCORE_MULTIPLIER;
        let score_text = score_label_text.to_owned() + &score.to_string();
        let score_surface = font.render(&score_text).blended(Color::RGBA(100, 149, 237, 255)).map_err(|e| e.to_string())?;
        let score_font_texture = texture_creator.create_texture_from_surface(&score_surface)
            .map_err(|e| e.to_string())?;

        let TextureQuery { width: score_text_width, height: score_text_height, .. } = score_font_texture.query();

        let score_text_padding = 64;
        let score_text_target = get_top_left_rect(score_text_width,
                                                  score_text_height,
                                                  WINDOW_WIDTH - score_text_padding,
                                                  WINDOW_HEIGHT - score_text_padding);

        canvas.copy(&score_font_texture, None, Some(score_text_target))?;

        // We update window's display
        canvas.present();

        // We sleep enough to get ~60 fps. If we don't call this, the program will take 100% of a CPU time.
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

fn update_ball_position(rect: &mut Rect, speed: Vector2, ball_position: &mut Vector2, delta_time: f32) {
    ball_position.x += speed.x * delta_time;
    rect.set_x(ball_position.x as i32);
    ball_position.y += speed.y * delta_time;
    rect.set_y(ball_position.y as i32);
}

fn check_for_victory_conditions(player_lives: i32, brick_count: i32) -> bool {
    let mut needs_restart = false;

    if player_lives == 0 {
        println!("GameOver!");
        needs_restart = true;
    }
    if brick_count == 0 {
        println!("Victory!");
        needs_restart = true;
    }

    needs_restart
}

fn restart_game(ball_rect: &mut Rect, ball_speed: &mut Vector2, player_lives: &mut i32, bricks_destroyed: &mut i32) {
    *player_lives = STARTING_PLAYER_LIVES;
    *bricks_destroyed = 0;
    reset_ball(ball_rect, ball_speed);
}

pub fn reset_ball(ball_rect: &mut Rect, ball_speed: &mut Vector2) {
    ball_rect.x = BALL_START_POSITION_X;
    ball_rect.y = BALL_START_POSITION_Y;

    ball_speed.x = BALL_START_SPEED_X;
    ball_speed.y = BALL_START_SPEED_Y;
}