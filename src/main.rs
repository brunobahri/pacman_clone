use macroquad::prelude::*;
use ::rand::thread_rng;
use ::rand::Rng;

#[macroquad::main("Pacman Clone")]
async fn main() {
    let mut pacman_x = 100.0;
    let mut pacman_y = 100.0;
    let mut score = 0;

    
    let mut pills = vec![];
    let mut rng = thread_rng();
    for _ in 0..20 {
        let x = rng.gen_range(0.0..screen_width());
        let y = rng.gen_range(0.0..screen_height());
        pills.push((x, y));
    }

    loop {
        clear_background(BLACK);

        
        if is_key_down(KeyCode::Right) {
            pacman_x += 2.0;
        }
        if is_key_down(KeyCode::Left) {
            pacman_x -= 2.0;
        }
        if is_key_down(KeyCode::Up) {
            pacman_y -= 2.0;
        }
        if is_key_down(KeyCode::Down) {
            pacman_y += 2.0;
        }

        
        draw_circle(pacman_x, pacman_y, 15.0, YELLOW);

        
        pills.retain(|&(x, y)| {
            draw_circle(x, y, 5.0, WHITE);
            let distance = ((pacman_x - x).powi(2) + (pacman_y - y).powi(2)).sqrt();
            if distance < 15.0 {
                score += 10; 
                false 
            } else {
                true
            }
        });

        
        draw_text(&format!("Pontuação: {}", score), 20.0, 20.0, 30.0, WHITE);

        next_frame().await;
    }
}
