#![warn(clippy::all, clippy::pedantic)]

use egui::CollapsingHeader;
use macroquad::{
    color::Color,
    input::{is_key_released, KeyCode},
    math::Vec2,
    shapes::draw_circle,
    time::get_frame_time,
    window::{clear_background, next_frame, Conf},
};

pub const CARROT_ORANGE: Color = Color {
    r: 247.0 / 255.0,
    g: 152.0 / 255.0,
    b: 36.0 / 255.0,
    a: 1.0,
};
pub const GUNMETAL: Color = Color {
    r: 49.0 / 255.0,
    g: 57.0 / 255.0,
    b: 60.0 / 255.0,
    a: 1.0,
};

const WINDOW_WIDTH: f32 = 1366.0;
const WINDOW_HEIGHT: f32 = 768.0;

#[derive(Debug)]
struct Ball {
    radius: f32,
    position: Vec2,
    velocity: Vec2,
}

impl Default for Ball {
    fn default() -> Ball {
        Ball {
            radius: 30.0,
            position: Vec2 {
                x: WINDOW_WIDTH / 2.0,
                y: WINDOW_HEIGHT / 2.0,
            },
            velocity: Vec2 {
                x: 500.0,
                y: -500.0,
            },
        }
    }
}

impl Ball {
    fn update(&mut self, delta: f32) {
        self.position += delta * self.velocity;
        self.handle_wall_collisions();
    }

    fn handle_wall_collisions(&mut self) {
        if self.left() < 0.0 {
            // shift the ball back into the window if it has slipped out
            let left_overlap = 0.0 - self.left();
            self.position.x += left_overlap;
            self.velocity.x *= -1.0;
        } else if self.right() > WINDOW_WIDTH {
            // shift the ball back into the window if it has slipped out
            let right_overlap = self.right() - WINDOW_WIDTH;
            self.position.x -= right_overlap;

            self.velocity.x *= -1.0;
        }
        if self.top() < 0.0 {
            // shift the ball back into the window if it has slipped out
            let top_overlap = 0.0 - self.top();
            self.position.y += top_overlap;

            self.velocity.y *= -1.0;
        } else if self.bottom() > WINDOW_HEIGHT {
            // shift the ball back into the window if it has slipped out
            let bottom_overlap = self.bottom() - WINDOW_HEIGHT;
            self.position.y -= bottom_overlap;

            self.velocity.y *= -1.0;
        }
    }

    fn left(&self) -> f32 {
        self.position.x - self.radius
    }

    fn right(&self) -> f32 {
        self.position.x + self.radius
    }

    fn top(&self) -> f32 {
        self.position.y - self.radius
    }

    fn bottom(&self) -> f32 {
        self.position.y + self.radius
    }
}

fn conf() -> Conf {
    #[allow(clippy::cast_possible_truncation)]
    Conf {
        window_title: String::from("Macroquad egui"),
        window_width: WINDOW_WIDTH as i32,
        window_height: WINDOW_HEIGHT as i32,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let mut ball = Ball::default();

    loop {
        let delta = get_frame_time();

        clear_background(GUNMETAL);

        if is_key_released(KeyCode::Escape) {
            break;
        }

        egui_macroquad::ui(|egui_ctx| {
            egui_ctx.set_pixels_per_point(4.0);
            egui::Window::new("Developer Tools").show(egui_ctx, |ui| {
                CollapsingHeader::new("Ball Physics")
                    .default_open(false)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label("Position");
                            ui.label(format!(
                                "x: {:.2} y: {:.2}",
                                ball.position.x, ball.position.y
                            ));
                        });
                        ui.horizontal(|ui| {
                            ui.label("Velocity");
                            ui.label(format!("x: {} y: {}", ball.velocity.x, ball.velocity.y));
                        });
                    });
                CollapsingHeader::new("Ball Shape")
                    .default_open(false)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label("Radius");
                            ui.add(egui::Slider::new(&mut ball.radius, 1.0..=100.0));
                        });
                    });
            });
        });

        draw_circle(ball.position.x, ball.position.y, ball.radius, CARROT_ORANGE);

        egui_macroquad::draw();

        ball.update(delta);

        next_frame().await;
    }
}
