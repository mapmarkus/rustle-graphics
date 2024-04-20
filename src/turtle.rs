use crate::draw::*;
use crate::units::{Angle, Distance, Pt};

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
    PenUp,
    Pivot { distance: Distance, arc: Angle },
    Repeat { count: u8, steps: Vec<Step> },
}

#[derive(Clone, Debug)]
pub struct Turtle {
    pub head: Angle,
    pub position: Pt,
}

impl Turtle {
    fn turn(&mut self, theta: Angle) {
        self.head = self.head.add(theta);
    }

    fn advance(&mut self, d: Distance) {
        self.position = (
            self.position.0 + d * self.head.cos(),
            self.position.1 + d * self.head.sin(),
        );
    }

    fn pivot(&mut self, r: Distance, arc: Angle) {
        let old_head = self.head.subtract(Angle::quarter_turn());
        self.turn(arc);
        let new_head = self.head.subtract(Angle::quarter_turn());
        let x = old_head.cos_r(-r) + new_head.cos_r(r);
        let y = old_head.sin_r(-r) + new_head.sin_r(r);
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
                        turtle.position.0
                            - turtle.head.subtract(Angle::quarter_turn()).cos_r(*distance),
                        turtle.position.1
                            - turtle.head.subtract(Angle::quarter_turn()).sin_r(*distance),
                        distance.clone(),
                        turtle.head.subtract(Angle::quarter_turn()),
                        turtle.head.add(*arc).subtract(Angle::quarter_turn()),
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
