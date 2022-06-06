use rusty_engine::prelude::*;


struct GameState {
    high_score: u32,
    current_score : u32,
    enemy_labels: Vec<String>,
    spawn_timer:Timer
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            high_score: 0,
            current_score : 0,
            enemy_labels: Vec::new(),
            spawn_timer: Timer::from_seconds(1.0,false),

        }
    }
}
fn main() {
    let mut game = Game::new();

    let player = game.add_sprite("player",SpritePreset::RacingCarBlue);
    player.translation = Vec2::new(200.0, 100.0);
    player.rotation = SOUTH_WEST;
    player.scale = 1.0;
    player.collision = true;

    let car1 = game.add_sprite("car1",SpritePreset::RacingCarYellow);
    car1.translation = Vec2::new(300.0,0.0);
    car1.collision = true;

    game.add_logic(game_logic);
    game.run(GameState::default());
}


fn game_logic(engine: &mut Engine, game_state:&mut GameState) {
    // game_state.current_score +=1;
    // println!("Current scroe: {}",game_state.current_score);


    for event in engine.collision_events.drain(..) {
        if event.state == CollisionState::Begin && event.pair.one_starts_with("player"){
            for label in [event.pair.0, event.pair.1] {
                if label != "player" {
                    engine.sprites.remove(&label);
                }
            }
            game_state.current_score += 1;
            println!("Current score: {}",game_state.current_score);
        }

    }

    let player = engine.sprites.get_mut("player").unwrap();
    const MOVEMENT_SPEED:f32 = 100.0;

    if engine.keyboard_state.pressed_any(&[KeyCode::Up,KeyCode::W]) {
        player.translation.y += MOVEMENT_SPEED *engine.delta_f32;

    }
    if engine.keyboard_state.pressed_any(&[KeyCode::Down,KeyCode::S]) {
        player.translation.y -= MOVEMENT_SPEED *engine.delta_f32;

    }
    if engine.keyboard_state.pressed_any(&[KeyCode::Right,KeyCode::D]) {
        player.translation.x += MOVEMENT_SPEED *engine.delta_f32;

    }
    if engine.keyboard_state.pressed_any(&[KeyCode::Left,KeyCode::A]) {
        player.translation.x -= MOVEMENT_SPEED *engine.delta_f32;

    }
}