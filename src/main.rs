use cgmath::Vector2;
use find_folder::Search;
use piston_window::*;
use sprite::*;
use std::rc::Rc;

mod player;
use player::*;

mod libs {
    pub mod controller;
    pub mod physics;
    pub mod transform;
}

use libs::controller::Controller;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Super Mario Bros", (640, 448))
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));

    let assets = Search::Parents(1).for_folder("assets").unwrap();

    let map_texture = Texture::from_path(
        &mut window.create_texture_context(),
        &assets.join("world_1-1.png"),
        Flip::None,
        &TextureSettings::new(),
    )
    .unwrap();
    let map_width = map_texture.get_width() as f64;
    let map_rc = Rc::new(map_texture);
    let mut map = Sprite::from_texture_rect(map_rc.clone(), [0.0, 0.0, map_width, 224.0]);
    map.set_scale(2.0, 2.0);
    map.set_position(map_width, 224.0);

    let texture = Rc::new(
        Texture::from_path(
            &mut window.create_texture_context(),
            &assets.join("players.png"),
            Flip::None,
            &TextureSettings::new(),
        )
        .unwrap(),
    );

    let mut sprite = Sprite::from_texture(texture.clone());
    sprite.set_position(16.0, 16.0);

    let mut animations = vec![];

    let mut player = Player::new();
    let mut controller = Controller::new();

    for x in (80..256).step_by(17) {
        let rect = [(x as f64) + 0.425, 34.45, 15.25, 15.25];
        animations.push(rect);
    }

    player.set_sprite(sprite);
    player.push_animation("idle", animations[0]);
    player.push_animation("jump", animations[5]);
    player.append_animation("walk", animations[0..4].to_vec());

    let mut map_pos = Vector2 { x: 0.0, y: 0.0 };

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _d| {
            clear([0.5, 1.0, 0.5, 1.0], g);
            map.draw(c.transform.trans(-map_pos.x, -map_pos.y), g);
            player.draw(c.transform, g);
        });

        if let Some(u) = e.update_args() {

            if !player.is_less_center(window.size())
                && player.dir_right()
                && map_pos.x < map_width * 2.0 - window.size().width
            {
                map_pos.x += player.get_vel_x();
            } else {
                player.update_position_x(u.dt * 100.0);
            }

            if map_pos.x < 0.0 {
                map_pos.x = 0.0;
            }

            if map_pos.x > map_width * 2.0 - window.size().width {
                map_pos.x = map_width * 2.0 - window.size().width;
            }

            controller.update(&mut player, u.dt * 100.0);
            player.update(u.dt * 100.0);
            player.set_inside_window(window.size());

        }

        if let Some(args) = e.button_args() {
            if let Button::Keyboard(key) = args.button {
                controller.keyboard_event(key, args.state);
            }
        }
    }
}