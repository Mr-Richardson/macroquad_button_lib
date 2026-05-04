use self::State::{Disabled, Hovered, Idle, Pressed};
use macroquad::color::Color;
use macroquad::input::{is_mouse_button_down, is_mouse_button_pressed};
use macroquad::math::{vec2, Vec2};
use macroquad::prelude::{mouse_position, Font};
use macroquad::shapes::{draw_ellipse, draw_rectangle};
use text_lib::text::Alignment;

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

pub struct Text<'a> {
    text: String,
    font: &'a Font,
    size: u16,
    color: Color,
}

pub struct Button<'a> {
    pos: Vec2,
    size: Vec2,
    shape: Shape,
    color: Color,
    toggle: bool,
    text: text_lib::text::Text<'a>,
    state: State,
}

impl<'a> Button<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        pos: Vec2,
        size: Vec2,
        shape: Shape,
        color: Color,
        toggle: bool,
        text: Text<'a>,
    ) -> Self {
        Button {
            pos,
            size,
            shape,
            color,
            text: text_lib::text::Text::new(pos, size.x * 0.9, text.text, text.font, Alignment { x: text_lib::text::AlignX::Center, y: text_lib::text::AlignY::Center }, text.size, text.color),
            toggle,
            state: Idle,
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
        self.text.draw();
    }

    pub fn set_pos(&mut self, pos: Vec2) {
        self.pos = pos;
        self.text.set_pos(pos);
    }

    pub fn set_size(&mut self, size: Vec2) {
        self.size = size;
        self.text.set_max_w(size.x * 0.9);
    }

    pub fn set_shape(&mut self, shape: Shape) {
        self.shape = shape;
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn set_text_text(&mut self, text: String) {
        self.text.set_text(text);
    }

    pub fn set_text_font(&mut self, font: &'a Font) {
        self.text.set_font(font);
    }

    pub fn set_text_size(&mut self, size: u16) {
        self.text.set_size(size);
    }

    pub fn set_text_color(&mut self, color: Color) {
        self.text.set_color(color);
    }

    pub fn set_toggle(&mut self, toggle: bool) {
        self.toggle = toggle;
    }

    pub fn get_state(&mut self) -> State {
        //TODO: cleaner implementation
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
}
