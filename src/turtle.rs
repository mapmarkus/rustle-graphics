use std::slice::Iter;

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
pub struct Trail {
    pub style: Style,
    pub path: Vec<PathIns>,
}

#[derive(Clone, Debug)]
pub enum PathIns {
    MoveTo {
        x: f64,
        y: f64,
    },
    LineTo {
        x: f64,
        y: f64,
    },
    Arc {
        center_x: f64,
        center_y: f64,
        radius: f64,
        start_angle: f64,
        end_angle: f64,
    },
}

#[derive(Clone, Debug)]
pub struct TurtleState {
    heading: Angle,
    position: Pt,
    saved: Vec<(Angle, Pt)>,
    journey: Vec<Trail>,
    is_drawing: bool,
}

impl Default for TurtleState {
    fn default() -> Self {
        TurtleState {
            heading: Angle::zero(),
            position: (0.0, 0.0),
            saved: vec![],
            journey: vec![],
            is_drawing: false,
        }
    }
}

impl<'a> TurtleState {
    pub fn new(heading: Angle, position: Pt) -> Self {
        let mut turtle_state = TurtleState::default();
        turtle_state.heading = heading;
        turtle_state.position = position;
        turtle_state
    }

    pub fn heading(&self) -> Angle {
        self.heading
    }

    pub fn position(&self) -> Pt {
        self.position
    }

    fn move_to(&mut self, x: Units, y: Units) {
        if let Some(trail) = self.journey.last_mut() {
            trail.path.push(PathIns::MoveTo { x, y });
        }
    }

    fn line_to(&mut self, x: Units, y: Units) {
        if let Some(trail) = self.trail() {
            trail.path.push(PathIns::LineTo { x, y })
        }
    }

    fn arc(
        &mut self,
        center_x: Units,
        center_y: Units,
        radius: Distance,
        start_angle: Angle,
        end_angle: Angle,
    ) {
        if let Some(trail) = self.trail() {
            trail.path.push(PathIns::Arc {
                center_x,
                center_y,
                radius,
                start_angle: start_angle.value(),
                end_angle: end_angle.value(),
            });
        }
    }

    fn trail(&mut self) -> Option<&mut Trail> {
        if self.is_drawing {
            self.journey.last_mut()
        } else {
            None
        }
    }

    pub fn iter(&'a self) -> Iter<'a, Trail> {
        self.journey.iter()
    }

    pub fn journey(&mut self, steps: &[Step]) {
        draw_turtle_in_drawable(self, steps);
    }

    fn turn(&mut self, theta: Angle) {
        self.heading = self.heading.add(theta);
    }

    fn go(&mut self, d: Distance) {
        self.position = (
            self.position.0 + self.heading.cos_r(d),
            self.position.1 + self.heading.sin_r(d),
        );
    }

    fn look_to(&mut self, pt: Pt) {
        self.heading = Angle::new(f64::atan2(pt.1 - self.position.1, pt.0 - self.position.0));
    }

    fn teleport(&mut self, pt: Pt) {
        self.position = pt;
    }

    fn being_trail(&mut self, s: Style) {
        self.journey.push(Trail {
            style: s,
            path: vec![],
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

fn draw_turtle_in_drawable(turtle: &mut TurtleState, steps: &[Step]) {
    for step in steps {
        match step {
            Step::Teleport(pt) => {
                turtle.teleport(*pt);
                turtle.move_to(turtle.position.0, turtle.position.1);
            }

            Step::LookTo(pt) => {
                turtle.look_to(*pt);
            }

            Step::PenDown(s) => {
                turtle.being_trail(s.clone());
                turtle.move_to(turtle.position.0, turtle.position.1);
                turtle.is_drawing = true;
            }

            Step::PenUp => {
                turtle.is_drawing = false;
            }

            Step::Save => {
                turtle.save();
            }

            Step::Restore => {
                turtle.restore();
                turtle.move_to(turtle.position.0, turtle.position.1);
            }

            Step::Go(d) => {
                turtle.go(*d);
                turtle.line_to(turtle.position.0, turtle.position.1);
            }

            Step::Turn(theta) => {
                turtle.turn(*theta);
            }

            Step::Pivot { distance, arc } => {
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
                    turtle.arc(pivot_x, pivot_y, *distance, tangent, tangent.add(*arc))
                } else {
                    turtle.move_to(x, y);
                    turtle.arc(pivot_x, pivot_y, *distance, dest_angle, tangent);
                    turtle.move_to(x, y);
                }

                turtle.turn(*arc);
                turtle.teleport((x, y));
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
