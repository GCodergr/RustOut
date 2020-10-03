use sdl2::rect::Rect;
use crate::game::{WINDOW_WIDTH, WINDOW_HEIGHT, Brick, reset_ball, Vector2};

pub fn check_for_collisions(rect: &mut Rect) {
    if rect.x < 0 {
        rect.set_x(0);
    }
    if rect.x > (WINDOW_WIDTH - rect.width()) as i32 {
        rect.set_x((WINDOW_WIDTH - rect.width()) as i32);
    }
}

pub fn resolve_ball_wall_collisions(ball_rect: &mut Rect, ball_speed: &mut Vector2, player_lives: &mut i32)  {
    if ball_rect.x < 0 {
        ball_rect.set_x(0);
        ball_speed.x = -ball_speed.x;
    }
    if ball_rect.right() > WINDOW_WIDTH as i32 {
        ball_rect.set_right(WINDOW_WIDTH as i32);
        ball_speed.x = -ball_speed.x;
    }
    if ball_rect.y < 0 {
        ball_rect.set_y(0);
        ball_speed.y = -ball_speed.y;
    }
    if ball_rect.bottom() > WINDOW_HEIGHT as i32 {
        ball_rect.set_bottom(WINDOW_HEIGHT as i32);
        ball_speed.y = -ball_speed.y;

        *player_lives = *player_lives -1;
        reset_ball(ball_rect, ball_speed);
    }
}

pub fn resolve_ball_paddle_collisions(ball_rect: Rect, ball_speed: &mut Vector2, paddle_rect: Rect){
    if ball_rect.has_intersection(paddle_rect){
        if (ball_rect.bottom() >= paddle_rect.top()) && ball_speed.y > 0.0  {
            ball_speed.y = -ball_speed.y;
        }

        // TODO: Make the ball change direction based on the position of the paddle (may even the speed if we make it variable)
    }
}

pub fn resolve_ball_brick_collisions(ball_rect: Rect, ball_speed: &mut Vector2, brick: &mut Brick, brick_count: &mut i32, bricks_destroyed: &mut i32){
    let mut collided = false;

    if brick.active && ball_rect.has_intersection(brick.rect)
    {
        if  (ball_rect.bottom() >= brick.rect.y) && ball_speed.y > 0.0 ||
            (ball_rect.y >= brick.rect.bottom()) && ball_speed.y < 0.0 {
            ball_speed.y = -ball_speed.y;
            collided = true;
        }
        if (ball_rect.right() >= brick.rect.x) && ball_speed.x > 0.0 ||
            (ball_rect.x >= brick.rect.x) && ball_speed.x < 0.0 {
            ball_speed.x = -ball_speed.x;
            collided = true;
        }
    }

    if collided
    {
        brick.active = false;
        *brick_count = *brick_count - 1;
        *bricks_destroyed = *bricks_destroyed + 1;
    }
}