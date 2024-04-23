use std::mem;

use crate::draw::*;
use crate::units::*;

#[derive(Clone, Debug)]
pub struct Style {
    pub color: String,
    pub width: Thickness,
}

impl Default for Style {
    fn default() -> Self {
        Style {
            color: "black".to_string(),
            width: 1.0,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Step {
    Teleport(Pt),
    LookTo(Pt),
    Go(Distance),
    Turn(Angle),
    PenDown(Style),
    PenUp,
    Save,
    Restore,
    Pivot { distance: Distance, arc: Angle },
    Repeat { count: u8, step: Box<Step> },
    Perform { steps: Vec<Step> },
}

#[derive(Clone, Debug)]
pub struct Trail<T> {
    pub style: Style,
    pub trail: T,
}

#[derive(Clone, Debug)]
struct TurtleState<T> {
    heading: Angle,
    position: Pt,
    saved: Vec<(Angle, Pt)>,
    style: Style,
    trail: T,
    journey: Vec<Trail<T>>,
    is_drawing: bool,
}

impl<T: Default> Default for TurtleState<T> {
    fn default() -> Self {
        TurtleState {
            heading: Angle::zero(),
            position: (0.0, 0.0),
            saved: vec![],
            style: Style::default(),
            trail: T::default(),
            journey: vec![],
            is_drawing: false,
        }
    }
}

impl<T: Default> TurtleState<T> {
    fn turn(&mut self, theta: Angle) {
        self.heading = self.heading.add(theta);
    }

    fn go(&mut self, d: Distance) {
        self.position = (
            self.position.0 + d * self.heading.cos(),
            self.position.1 + d * self.heading.sin(),
        );
    }

    fn look_to(&mut self, pt: Pt) {
        self.heading = Angle::new(f64::atan2(pt.1 - self.position.1, pt.0 - self.position.0));
    }

    fn teleport(&mut self, pt: Pt) {
        self.position = pt;
    }

    fn being_trail(&mut self, s: Style) {
        let old_style = mem::replace(&mut self.style, s);
        let old_trail = mem::replace(&mut self.trail, T::default());
        self.journey.push(Trail {
            style: old_style,
            trail: old_trail,
        });
    }

    fn save(&mut self) {
        self.saved.push((self.heading, self.position));
    }

    fn restore(&mut self) {
        if let Some((ang, pos)) = self.saved.pop() {
            self.heading = ang;
            self.position = pos;
        }
    }
}

pub fn draw_turtle<T: Drawable + Default>(steps: &[Step]) -> Vec<Trail<T>> {
    let mut turtle_state: TurtleState<T> = TurtleState::default();
    draw_turtle_in_drawable(&mut turtle_state, steps);
    let mut journey = turtle_state.journey;
    journey.push(Trail {
        style: turtle_state.style,
        trail: turtle_state.trail,
    });
    return journey;
}

fn draw_turtle_in_drawable<T: Drawable + Default>(turtle: &mut TurtleState<T>, steps: &[Step]) {
    for step in steps {
        match step {
            Step::Teleport(pt) => {
                turtle.teleport(*pt);
                turtle.trail.move_to(pt.0, pt.1);
            }
            Step::LookTo(pt) => {
                turtle.look_to(*pt);
            }
            Step::PenDown(s) => {
                if turtle.is_drawing {
                    turtle.being_trail(s.clone());
                } else {
                    turtle.style = s.clone();
                    turtle.is_drawing = true;
                }
            }
            Step::PenUp => {
                if turtle.is_drawing {
                    turtle.being_trail(Style::default());
                    turtle.is_drawing = false;
                }
            }
            Step::Save => {
                turtle.save();
            }
            Step::Restore => {
                turtle.restore();
                if turtle.is_drawing {
                    turtle.trail.move_to(turtle.position.0, turtle.position.1);
                }
            }
            Step::Go(d) => {
                turtle.go(*d);
                if turtle.is_drawing {
                    turtle.trail.line_to(turtle.position.0, turtle.position.1);
                }
            }
            Step::Turn(theta) => {
                turtle.turn(*theta);
            }
            Step::Pivot { distance, arc } => {
                if turtle.is_drawing {
                    // Subtracting or adding PI/2 is necessary to account for the fact that we want the turtle to be tangent to the "pivot arc"
                    // If we don't do it, the turtle will be facing outward (normal) to the pivot circle, and that would not achieve
                    // the smooth effect of a turtle pivoting around a point.
                    let tangent = if arc.value() > 0.0 {
                        turtle.heading.subtract(Angle::quarter_turn())
                    } else {
                        turtle.heading.add(Angle::quarter_turn())
                    };

                    let dest_angle = tangent.add(*arc);
                    let pivot_x = turtle.position.0 - tangent.cos_r(*distance);
                    let pivot_y = turtle.position.1 - tangent.sin_r(*distance);
                    let x = pivot_x + dest_angle.cos_r(*distance);
                    let y = pivot_y + dest_angle.sin_r(*distance);

                    if arc.value() > 0.0 {
                        turtle.trail.arc(
                            pivot_x,
                            pivot_y,
                            *distance,
                            tangent.value(),
                            tangent.add(*arc).value(),
                        )
                    } else {
                        turtle.trail.move_to(x, y);
                        turtle.trail.arc(
                            pivot_x,
                            pivot_y,
                            *distance,
                            dest_angle.value(),
                            tangent.value(),
                        );
                        turtle.trail.move_to(x, y);
                    }

                    turtle.turn(*arc);
                    turtle.teleport((x, y));
                }
            }
            Step::Perform { steps } => {
                draw_turtle_in_drawable(turtle, steps);
            }
            Step::Repeat { count, step } => {
                let unboxed_step = (*step).as_ref();
                for _ in 0..*count {
                    draw_turtle_in_drawable(turtle, &[unboxed_step.clone()]);
                }
            }
        }
    }
}
