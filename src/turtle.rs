use std::f64::consts::PI;

use crate::draw::*;

pub type Thickness = f64;

#[derive(Clone, Debug)]
pub struct Style {
    pub color: String,
    pub width: Thickness,
}

#[derive(Clone, Debug)]
pub enum Step {
    Move(Distance),
    Turn(Angle),
    PenDown(Style),
    Pivot { distance: Distance, arc: Angle },
    PenUp,
    Repeat { count: u8, steps: Vec<Step> },
}

#[derive(Clone, Debug)]
pub struct Turtle {
    pub head: Angle,
    pub position: Pt,
}

impl Turtle {
    fn turn(&mut self, theta: Angle) {
        self.head = (self.head + theta) % (2.0 * PI);
    }

    fn advance(&mut self, d: Distance) {
        self.position = (
            self.position.0 + d * f64::cos(self.head),
            self.position.1 + d * f64::sin(self.head),
        );
    }

    fn pivot(&mut self, r: Distance, arc: Angle) {
        let head = self.head - PI / 2.0;
        self.turn(arc);
        let new_head = self.head - PI / 2.0;
        let x = -r * f64::cos(head) + r * f64::cos(new_head);
        let y = -r * f64::sin(head) + r * f64::sin(new_head);
        self.position = (self.position.0 + x, self.position.1 + y);
    }
}

pub fn draw_turtle<T: Drawable + Default>(turtle: &mut Turtle, steps: &[Step]) -> Vec<(Style, T)> {
    let mut trails = vec![];
    let mut path = None;
    draw_turtle_in_drawable(turtle, steps, &mut path, &mut trails);
    if path.is_some() {
        let cp = path.take().unwrap();
        trails.push(cp);
    }
    return trails;
}

fn draw_turtle_in_drawable<T: Drawable + Default>(
    turtle: &mut Turtle,
    steps: &[Step],
    path: &mut Option<(Style, T)>,
    trails: &mut Vec<(Style, T)>,
) {
    for step in steps {
        match step {
            Step::PenDown(s) => {
                if path.is_some() {
                    let cp = path.take().unwrap();
                    trails.push(cp);
                }
                let mut new_p = T::default();
                new_p.move_to(turtle.position.0, turtle.position.1);
                *path = Some((s.clone(), new_p));
            }
            Step::Move(d) => {
                turtle.advance(*d);
                // console::log_1(
                //     &format!("Move. Line to {} {}", turtle.position.0, turtle.position.1).into(),
                // );
                if let Some(ref mut p) = path {
                    p.1.line_to(turtle.position.0, turtle.position.1);
                }
            }
            Step::Turn(theta) => {
                // console::log_1(&format!("Turn {} rad", theta).into());
                turtle.turn(*theta);
                // console::log_1(&format!("Turtle is heading {} rad", turtle.head).into());
            }
            Step::Pivot { distance, arc } => {
                if let Some(ref mut p) = path {
                    p.1.arc(
                        turtle.position.0 - distance * f64::cos(turtle.head - PI / 2.0),
                        turtle.position.1 - distance * f64::sin(turtle.head - PI / 2.0),
                        distance.clone(),
                        turtle.head - PI / 2.0,
                        turtle.head + arc - PI / 2.0,
                    )
                }
                turtle.pivot(*distance, *arc);
            }
            Step::PenUp => {
                if path.is_some() {
                    let cp = path.take().unwrap();
                    trails.push(cp);
                }
                *path = None;
            }
            Step::Repeat { count, steps } => {
                for _ in 0..*count {
                    draw_turtle_in_drawable(turtle, steps, path, trails);
                }
            }
        }
    }
}
