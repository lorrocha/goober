extern crate piston_window;

mod entity;

use piston_window::*;
use entity::Entity;

#[derive(Debug, Clone)]
pub struct Point(f64, f64);

const MU: f64 = 0.93;
const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

fn main() {
    let title = "Goober";
    let mut arrows = Entity::new(Point(615.0, 455.0), [0.3, 0.0, 0.7, 0.5], 25.0, 25.0);
    let mut wasd = Entity::new(Point(0.0, 0.0), [0.7, 0.0, 0.3, 0.5], 25.0, 25.0);
    let mut arrows_points: i32 = 0;
    let mut wasd_points: i32 = 0;
    let mut goal_block = Entity::random_new();

    let mut window: PistonWindow = WindowSettings::new(title, [WIDTH, HEIGHT])
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));

        let font = "assets/FiraSans-Regular.ttf";
        let factory = window.factory.clone();
        let mut glyphs = Glyphs::new(font, factory).unwrap();

    while let Some(e) = window.next() {

        if let Some(_) = e.render_args() {
            window.draw_2d(&e, |c, g| {
                clear([0.0, 0.0, 0.0, 1.0], g);
                rectangle(arrows.color, arrows.geometry(), c.transform, g);
                rectangle(wasd.color, wasd.geometry(), c.transform, g);
                rectangle(goal_block.color, goal_block.geometry(), c.transform, g);

                let arrows_overlap = goal_block.overlapping(&arrows);
                let wasd_overlap = goal_block.overlapping(&wasd);
                if arrows_overlap {
                    arrows_points += 1;
                } else if wasd_overlap {
                    wasd_points += 1;
                }

                if arrows_overlap || wasd_overlap {
                    goal_block = Entity::random_new();
                }

                let display_text = format!("{} | {}", wasd_points, arrows_points);

                let transform = c.transform.trans(40.0, 40.0);
                text::Text::new_color([0.0, 1.0, 0.0, 1.0], 32).draw(
                    &display_text,
                    &mut glyphs,
                    &c.draw_state,
                    transform, g
                );
            });
        }

        if let Some(button) = e.press_args() {
            match button {
                Button::Keyboard(Key::Up)    => arrows.adjust_dy(-1.0),
                Button::Keyboard(Key::Down)  => arrows.adjust_dy(1.0),
                Button::Keyboard(Key::Left)  => arrows.adjust_dx(-1.0),
                Button::Keyboard(Key::Right) => arrows.adjust_dx(1.0),
                Button::Keyboard(Key::W)     => wasd.adjust_dy(-1.0),
                Button::Keyboard(Key::S)     => wasd.adjust_dy(1.0),
                Button::Keyboard(Key::A)     => wasd.adjust_dx(-1.0),
                Button::Keyboard(Key::D)     => wasd.adjust_dx(1.0),
                _ => ()
            }
        }

        if let Some(_) = e.update_args() {
            arrows.nudge();
            wasd.nudge();
        }
    }
}
