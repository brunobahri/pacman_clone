use macroquad::prelude::*;

#[macroquad::main("Pacman Clone")]
async fn main() {
    let mut pacman_x = 100.0;
    let mut pacman_y = 100.0;

    loop {
        clear_background(BLACK);

        // Movimento do Pacman
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

        // Desenhar Pacman
        draw_circle(pacman_x, pacman_y, 15.0, YELLOW);

        next_frame().await;
    }
}
