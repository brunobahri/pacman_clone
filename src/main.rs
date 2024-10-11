use macroquad::prelude::*;
use ::rand::thread_rng;
use ::rand::Rng;

#[macroquad::main("Pacman Clone")]
async fn main() {
    let mut pacman_x = 100.0;
    let mut pacman_y = 100.0;
    let mut score = 0;

    // Gerar pílulas aleatoriamente
    let mut pills = vec![];
    let mut rng = thread_rng();
    for _ in 0..20 {
        let x = rng.gen_range(0.0..screen_width());
        let y = rng.gen_range(0.0..screen_height());
        pills.push((x, y));
    }

    // Inimigo inicial
    let mut enemy_x = rng.gen_range(0.0..screen_width());
    let mut enemy_y = rng.gen_range(0.0..screen_height());
    let mut enemy_dir_x = rng.gen_range(-1.0..1.0);
    let mut enemy_dir_y = rng.gen_range(-1.0..1.0);

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

        // Movimento do Inimigo
        enemy_x += enemy_dir_x * 2.0;
        enemy_y += enemy_dir_y * 2.0;

        // Verificar colisão do inimigo com as bordas e mudar direção
        if enemy_x < 0.0 || enemy_x > screen_width() {
            enemy_dir_x = -enemy_dir_x;
        }
        if enemy_y < 0.0 || enemy_y > screen_height() {
            enemy_dir_y = -enemy_dir_y;
        }

        // Desenhar Pacman
        draw_circle(pacman_x, pacman_y, 15.0, YELLOW);

        // Desenhar Inimigo
        draw_circle(enemy_x, enemy_y, 15.0, RED);

        // Desenhar Pílulas
        pills.retain(|&(x, y)| {
            draw_circle(x, y, 5.0, WHITE);
            let distance = ((pacman_x - x).powi(2) + (pacman_y - y).powi(2)).sqrt();
            if distance < 15.0 {
                score += 10; // Aumenta a pontuação quando o Pacman coleta uma pílula
                false // Remove a pílula coletada
            } else {
                true
            }
        });

        // Verificar colisão do Pacman com o inimigo
        let distance_to_enemy = ((pacman_x - enemy_x).powi(2) + (pacman_y - enemy_y).powi(2)).sqrt();
        if distance_to_enemy < 15.0 {
            // Exibir mensagem de Game Over
            draw_text("Game Over!", screen_width() / 2.0 - 80.0, screen_height() / 2.0, 50.0, WHITE);
            next_frame().await;
            continue;
        }

        // Desenhar Pontuação
        draw_text(&format!("Pontuação: {}", score), 20.0, 20.0, 30.0, WHITE);

        next_frame().await;
        
    }
}
