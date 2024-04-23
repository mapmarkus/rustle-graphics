use crate::draw::*;
use crate::units::*;

#[derive(Clone, Debug)]
pub struct Style {
    pub color: String,
    pub width: Thickness,
}

#[derive(Clone, Debug)]
pub enum Step {
    Teleport(Pt),
    LookTo(Pt),
    Go(Distance),
    Turn(Angle),
    PenDown(Style),
    PenUp,
    Pivot { distance: Distance, arc: Angle },
    Repeat { count: u8, step: Box<Step> },
    Perform { steps: Vec<Step> },
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

    fn go(&mut self, d: Distance) {
        self.position = (
            self.position.0 + d * self.head.cos(),
            self.position.1 + d * self.head.sin(),
        );
    }

    fn look_to(&mut self, pt: Pt) {
        self.head = Angle::new(f64::atan2(pt.1 - self.position.1, pt.0 - self.position.1));
    }

    fn teleport(&mut self, pt: Pt) {
        self.position = pt;
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
            Step::Teleport(pt) => {
                turtle.teleport(*pt);
                if let Some(ref mut p) = path {
                    p.1.move_to(pt.0, pt.1);
                }
            }
            Step::LookTo(pt) => {
                turtle.look_to(*pt);
            }
            Step::PenDown(s) => {
                if path.is_some() {
                    let cp = path.take().unwrap();
                    trails.push(cp);
                }
                let new_p = T::default();
                *path = Some((s.clone(), new_p));
            }
            Step::Go(d) => {
                turtle.go(*d);
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
                    // Subtracting or adding PI/2 is necessary to account for the fact that we want the turtle to be tangent to the "pivot arc"
                    // If we don't do it, the turtle will be facing outward (normal) to the pivot circle, and that would not achieve
                    // the smooth effect of a turtle pivoting around a point.
                    let tangent_head = if arc.value() > 0.0 {
                        turtle.head.subtract(Angle::quarter_turn())
                    } else {
                        turtle.head.add(Angle::quarter_turn())
                    };

                    let dest_angle = tangent_head.add(*arc);
                    let pivot_x = turtle.position.0 - tangent_head.cos_r(*distance);
                    let pivot_y = turtle.position.1 - tangent_head.sin_r(*distance);
                    let x = pivot_x + dest_angle.cos_r(*distance);
                    let y = pivot_y + dest_angle.sin_r(*distance);

                    if arc.value() > 0.0 {
                        p.1.arc(
                            pivot_x,
                            pivot_y,
                            *distance,
                            tangent_head.value(),
                            tangent_head.add(*arc).value(),
                        )
                    } else {
                        p.1.move_to(x, y);
                        p.1.arc(
                            pivot_x,
                            pivot_y,
                            *distance,
                            dest_angle.value(),
                            tangent_head.value(),
                        );
                        p.1.move_to(x, y);
                    }

                    turtle.turn(*arc);
                    turtle.teleport((x, y));
                }
            }
            Step::PenUp => {
                if path.is_some() {
                    let cp = path.take().unwrap();
                    trails.push(cp);
                }
                *path = None;
            }
            Step::Perform { steps } => {
                draw_turtle_in_drawable(turtle, steps, path, trails);
            }
            Step::Repeat { count, step } => {
                let unboxed_step = (*step).as_ref();
                for _ in 0..*count {
                    draw_turtle_in_drawable(turtle, &[unboxed_step.clone()], path, trails);
                }
            }
        }
    }
}
