
//! Back-end agnostic controller events.

use { Event, Input, Motion };

/// Components of a controller button event. Not guaranteed consistent across
/// backends.
#[derive(Copy, Clone, RustcDecodable, RustcEncodable, PartialEq, Eq, Debug, Hash)]
pub struct ControllerButton {
    /// Which controller was the button on.
    pub id: i32,
    /// Which button was pressed.
    pub button: u8,
}

impl ControllerButton {
    /// Create a new ControllerButton object. Intended for use by backends when
    /// emitting events.
    pub fn new(id: i32, button: u8) -> Self {
        ControllerButton {
            id: id,
            button: button,
        }
    }
}

/// Components of a controller axis move event. Not guaranteed consistent across
/// backends.
#[derive(Copy, Clone, RustcDecodable, RustcEncodable, PartialEq, Debug)]
pub struct ControllerAxisArgs {
    /// Which controller moved.
    pub id: i32,
    /// The axis that moved.
    pub axis: u8,
    /// Position of the controller. Usually [-1.0, 1.0], though backends may use
    /// a different range for various devices.
    pub position: f64
}

impl ControllerAxisArgs {
    /// Create a new ControllerAxisArgs object. Intended for use by backends when
    /// emitting events.
    pub fn new(id: i32, axis: u8, position: f64) -> Self {
        ControllerAxisArgs {
            id: id,
            axis: axis,
            position: position,
        }
    }
}

/// The position of a controller axis changed.
pub trait ControllerAxisEvent: Sized {
    /// Creates a controller axis event.
    fn from_controller_axis_args(
        args: ControllerAxisArgs,
        old_event: &Self
    ) -> Option<Self>;
    /// Calls closure if this is a controller axis event.
    fn controller_axis<U, F>(&self, f: F) -> Option<U>
        where F: FnMut(ControllerAxisArgs) -> U;
    /// Returns controller axis arguments.
    fn controller_axis_args(&self) -> Option<ControllerAxisArgs> {
        self.controller_axis(|args| args)
    }
}

/* TODO: Enable when specialization gets stable.
impl<T: GenericEvent> ControllerAxisEvent for T {
    fn from_controller_axis_args(
        args: ControllerAxisArgs,
        old_event: &Self
    ) -> Option<Self> {
        GenericEvent::from_args(CONTROLLER_AXIS, &args as &Any, old_event)
    }

    fn controller_axis<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(ControllerAxisArgs) -> U
    {
        if self.event_id() != CONTROLLER_AXIS {
            return None;
        }
        self.with_args(|any| {
            if let Some(&args) = any.downcast_ref::<ControllerAxisArgs>() {
                Some(f(args))
            } else {
                panic!("Expected ControllerAxisArgs")
            }
        })
    }
}
*/

impl ControllerAxisEvent for Input {
    fn from_controller_axis_args(
        args: ControllerAxisArgs,
        _old_event: &Self
    ) -> Option<Self> {
        Some(Input::Move(Motion::ControllerAxis(args)))
    }

    fn controller_axis<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(ControllerAxisArgs) -> U
    {
        match *self {
            Input::Move(Motion::ControllerAxis(args)) => Some(f(args)),
            _ => None
        }
    }
}

impl<I: ControllerAxisEvent> ControllerAxisEvent for Event<I> {
    fn from_controller_axis_args(
        args: ControllerAxisArgs,
        old_event: &Self
    ) -> Option<Self> {
        if let &Event::Input(ref old_input) = old_event {
            <I as ControllerAxisEvent>::from_controller_axis_args(args, old_input)
                .map(|x| Event::Input(x))
        } else {
            None
        }
    }

    fn controller_axis<U, F>(&self, f: F) -> Option<U>
        where F: FnMut(ControllerAxisArgs) -> U
    {
        match *self {
            Event::Input(ref x) => x.controller_axis(f),
            _ => None
        }
    }
}

#[cfg(test)]
mod controller_axis_tests {
    use super::*;

    #[test]
    fn test_input_controller_axis() {
        use super::super::{ Input, Motion };

        let e = Input::Move(Motion::ControllerAxis(
            ControllerAxisArgs::new(0, 1, 0.9)));
        let a: Option<Input> = ControllerAxisEvent::from_controller_axis_args(
            ControllerAxisArgs::new(0, 1, 0.9), &e);
        let b: Option<Input> = a.clone().unwrap().controller_axis(|cnt|
            ControllerAxisEvent::from_controller_axis_args(
                ControllerAxisArgs::new(cnt.id, cnt.axis, cnt.position),
                a.as_ref().unwrap())).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn test_event_controller_axis() {
        use Event;
        use super::super::{ Input, Motion };

        let e = Event::Input(Input::Move(Motion::ControllerAxis(
            ControllerAxisArgs::new(0, 1, 0.9))));
        let a: Option<Event> = ControllerAxisEvent::from_controller_axis_args(
            ControllerAxisArgs::new(0, 1, 0.9), &e);
        let b: Option<Event> = a.clone().unwrap().controller_axis(|cnt|
            ControllerAxisEvent::from_controller_axis_args(
                ControllerAxisArgs::new(cnt.id, cnt.axis, cnt.position),
                a.as_ref().unwrap())).unwrap();
        assert_eq!(a, b);
    }
}
