use self::State::{Disabled, Hovered, Idle, Pressed};
use macroquad::color::Color;
use macroquad::input::{is_mouse_button_down, is_mouse_button_pressed};
use macroquad::math::{Vec2, vec2};
use macroquad::prelude::{TextDimensions, mouse_position};
use macroquad::shapes::{draw_ellipse, draw_rectangle};
use macroquad::text::{draw_text, measure_text};

pub enum Shape {
    Rectangle,
    Ellipse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Idle,
    Hovered,
    Pressed,
    Disabled,
}
pub struct Button {
    pos: Vec2,
    size: Vec2,
    shape: Shape,
    color: Color,
    text: String,
    toggle: bool,
    state: State,
    text_dimensions: TextDimensions,
    text_size: f32,
}

impl Button {
    pub fn new(
        pos: Vec2,
        size: Vec2,
        shape: Shape,
        color: Color,
        text: String,
        toggle: bool,
    ) -> Self {
        let text_size: f32 = (size.x / measure_text(&text, None, 1, 1.0).width)
            .min(size.y / measure_text(&text, None, 1, 1.0).height);
        Button {
            pos,
            size,
            shape,
            color,
            text: text.clone(),
            toggle,
            state: Idle,
            text_dimensions: measure_text(&text, None, text_size as u16, 1.0),
            text_size,
        }
    }

    pub fn render(&self) {
        match self.shape {
            Shape::Rectangle => draw_rectangle(
                self.pos.x - self.size.x * 0.5,
                self.pos.y - self.size.y * 0.5,
                self.size.x,
                self.size.y,
                self.color,
            ),
            Shape::Ellipse => draw_ellipse(
                self.pos.x,
                self.pos.y,
                self.size.x * 0.5,
                self.size.y * 0.5,
                0.0,
                self.color,
            ),
        }
        draw_text(
            &self.text,
            self.pos.x - self.text_dimensions.width * 0.5,
            self.pos.y + self.text_dimensions.height * 0.5,
            self.text_size,
            Color {
                r: 1.0 - self.color.r,
                g: 1.0 - self.color.g,
                b: 1.0 - self.color.b,
                a: 1.0,
            },
        );
    }

    pub fn get_state(&mut self) -> State {
        if self.state != Disabled {
            let mouse_pos: Vec2 = vec2(mouse_position().0, mouse_position().1);
            let mut over: bool = false;
            match self.shape {
                Shape::Ellipse => {
                    if 1.0
                        >= (mouse_pos.x - self.pos.x) * (mouse_pos.x - self.pos.x)
                            / ((self.size.x * 0.5) * (self.size.x * 0.5))
                            + (mouse_pos.y - self.pos.y) * (mouse_pos.y - self.pos.y)
                                / ((self.size.y * 0.5) * (self.size.y * 0.5))
                    {
                        over = true;
                    }
                }
                Shape::Rectangle => {
                    if (mouse_pos.x - self.pos.x).abs() <= self.size.x * 0.5
                        && (mouse_pos.y - self.pos.y).abs() <= self.size.y * 0.5
                    {
                        over = true;
                    }
                }
            }
            if self.toggle {
                if over {
                    if is_mouse_button_pressed(macroquad::input::MouseButton::Left) {
                        if self.state == Pressed {
                            self.state = Hovered;
                        } else {
                            self.state = Pressed;
                        }
                    } else {
                        if self.state != Pressed {
                            self.state = Hovered;
                        }
                    }
                } else {
                    if self.state != Pressed {
                        self.state = Idle;
                    }
                }
            } else {
                if over {
                    if is_mouse_button_down(macroquad::input::MouseButton::Left) {
                        self.state = Pressed;
                    } else {
                        self.state = Hovered;
                    }
                } else {
                    self.state = Idle;
                }
            }
        }
        self.state
    }

    pub fn disable(&mut self) {
        self.state = Disabled;
    }

    pub fn set_text(&mut self, text: String) {
        self.text_size = (self.size.x / measure_text(&text, None, 1, 1.0).width)
            .min(self.size.y / measure_text(&text, None, 1, 1.0).height);
        self.text_dimensions = measure_text(&text, None, self.text_size as u16, 1.0);
        self.text = text;
    }
}
