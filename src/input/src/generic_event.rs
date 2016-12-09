//! Trait for generic events

use std::borrow::ToOwned;
use std::any::Any;

use {AfterRenderEvent, ControllerAxisEvent, CursorEvent, FocusEvent, IdleEvent,
     MouseCursorEvent, MouseRelativeEvent, MouseScrollEvent,
     PressEvent, ReleaseEvent, RenderEvent, ResizeEvent,
     TextEvent, TouchEvent, UpdateEvent};
use {AfterRenderArgs, ControllerAxisArgs, Button, Event, EventId, IdleArgs, Input,
     Motion, RenderArgs, TouchArgs, UpdateArgs};
use {AFTER_RENDER, CONTROLLER_AXIS, CURSOR, FOCUS, CLOSE, IDLE, MOUSE_CURSOR,
     MOUSE_RELATIVE, MOUSE_SCROLL, PRESS, RENDER, RELEASE, RESIZE,
     TEXT, TOUCH, UPDATE};

/// Implemented by all events
pub trait GenericEvent: Sized +
    AfterRenderEvent + ControllerAxisEvent + CursorEvent + FocusEvent + IdleEvent +
    MouseCursorEvent + MouseRelativeEvent + MouseScrollEvent +
    PressEvent + ReleaseEvent + RenderEvent + ResizeEvent +
    TextEvent + TouchEvent + UpdateEvent {
    /// The id of this event.
    fn event_id(&self) -> EventId;
    /// Calls closure with arguments
    fn with_args<'a, F, U>(&'a self, f: F) -> U
        where F: FnMut(&Any) -> U
    ;
    /// Converts from arguments to `Self`.
    ///
    /// Returns `None` if old event is not same kind.
    fn from_args(event_id: EventId, any: &Any, old_event: &Self) -> Option<Self>;
}

impl GenericEvent for Input {
    fn event_id(&self) -> EventId {
        match self {
            &Input::Cursor(_) => CURSOR,
            &Input::Focus(_) => FOCUS,
            &Input::Close => CLOSE,
            &Input::Move(Motion::MouseCursor(_, _)) => MOUSE_CURSOR,
            &Input::Move(Motion::MouseRelative(_, _)) => MOUSE_RELATIVE,
            &Input::Move(Motion::MouseScroll(_, _)) => MOUSE_SCROLL,
            &Input::Move(Motion::ControllerAxis(_)) => CONTROLLER_AXIS,
            &Input::Move(Motion::Touch(_)) => TOUCH,
            &Input::Press(_) => PRESS,
            &Input::Release(_) => RELEASE,
            &Input::Resize(_, _) => RESIZE,
            &Input::Text(_) => TEXT,
        }
    }

    fn with_args<'a, F, U>(&'a self, mut f: F) -> U
        where F: FnMut(&Any) -> U
    {
        match self {
            &Input::Cursor(cursor) =>
                f(&cursor as &Any),
            &Input::Focus(focused) =>
                f(&focused as &Any),
            &Input::Close =>
                f(&(None as Option<()>)),
            &Input::Move(Motion::ControllerAxis(args)) =>
                f(&args as &Any),
            &Input::Move(Motion::MouseCursor(x, y)) =>
                f(&(x, y) as &Any),
            &Input::Move(Motion::MouseRelative(x, y)) =>
                f(&(x, y) as &Any),
            &Input::Move(Motion::MouseScroll(x, y)) =>
                f(&(x, y) as &Any),
            &Input::Move(Motion::Touch(args)) =>
                f(&args as &Any),
            &Input::Press(button) =>
                f(&button as &Any),
            &Input::Release(button) =>
                f(&button as &Any),
            &Input::Resize(w, h) =>
                f(&(w, h) as &Any),
            &Input::Text(ref text) =>
                f(text as &Any),
        }
    }

    fn from_args(event_id: EventId, any: &Any, _old_event: &Self) -> Option<Self> {
        match event_id {
            x if x == CONTROLLER_AXIS => {
                if let Some(&args) = any.downcast_ref::<ControllerAxisArgs>() {
                    Some(Input::Move(Motion::ControllerAxis(args)))
                } else {
                    panic!("Expected ControllerAxisArgs")
                }
            }
            x if x == CURSOR => {
                if let Some(&cursor) = any.downcast_ref::<bool>() {
                    Some(Input::Cursor(cursor))
                } else {
                    panic!("Expected bool")
                }
            }
            x if x == FOCUS => {
                if let Some(&focused) = any.downcast_ref::<bool>() {
                    Some(Input::Focus(focused))
                } else {
                    panic!("Expected bool")
                }
            }
            x if x == MOUSE_CURSOR => {
                if let Some(&(x, y)) = any.downcast_ref::<(f64, f64)>() {
                    Some(Input::Move(Motion::MouseCursor(x, y)))
                } else {
                    panic!("Expected (f64, f64)")
                }
            }
            x if x == MOUSE_RELATIVE => {
                if let Some(&(x, y)) = any.downcast_ref::<(f64, f64)>() {
                    Some(Input::Move(Motion::MouseRelative(x, y)))
                } else {
                    panic!("Expected (f64, f64)")
                }
            }
            x if x == MOUSE_SCROLL => {
                if let Some(&(x, y)) = any.downcast_ref::<(f64, f64)>() {
                    Some(Input::Move(Motion::MouseScroll(x, y)))
                } else {
                    panic!("Expected (f64, f64)")
                }
            }
            x if x == PRESS => {
                if let Some(&button) = any.downcast_ref::<Button>() {
                    Some(Input::Press(button))
                } else {
                    panic!("Expected Button")
                }
            }
            x if x == RELEASE => {
                if let Some(&button) = any.downcast_ref::<Button>() {
                    Some(Input::Release(button))
                } else {
                    panic!("Expected Button")
                }
            }
            x if x == RESIZE => {
                if let Some(&(w, h)) = any.downcast_ref::<(u32, u32)>() {
                    Some(Input::Resize(w as u32, h as u32))
                } else {
                    panic!("Expected (u32, u32))")
                }
            }
            x if x == TEXT => {
                if let Some(text) = any.downcast_ref::<String>() {
                    Some(Input::Text(text.to_owned()))
                } else {
                    panic!("Expected &str")
                }
            }
            x if x == TOUCH => {
                if let Some(&args) = any.downcast_ref::<TouchArgs>() {
                    Some(Input::Move(Motion::Touch(args)))
                } else {
                    panic!("Expected TouchArgs")
                }
            }
            _ => { return None; }
        }
    }
}

impl<I: GenericEvent> GenericEvent for Event<I> {
    fn event_id(&self) -> EventId {
        match self {
            &Event::Update(_) => UPDATE,
            &Event::Render(_) => RENDER,
            &Event::AfterRender(_) => AFTER_RENDER,
            &Event::Idle(_) => IDLE,
            &Event::Input(ref input) => {
                input.event_id()
            }
        }
    }

    fn with_args<'a, F, U>(&'a self, mut f: F) -> U
        where F: FnMut(&Any) -> U
    {
        match self {
            &Event::Update(ref args) => {
                f(args as &Any)
            }
            &Event::Render(ref args) => {
                f(args as &Any)
            }
            &Event::AfterRender(ref args) => {
                f(args as &Any)
            }
            &Event::Idle(ref args) => {
                f(args as &Any)
            }
            &Event::Input(ref input) => {
                input.with_args(f)
            }
        }
    }

    fn from_args(event_id: EventId, any: &Any, old_event: &Self) -> Option<Self> {
        match event_id {
            x if x == UPDATE => {
                if let Some(&args) = any.downcast_ref::<UpdateArgs>() {
                    Some(Event::Update(args))
                } else {
                    panic!("Expected UpdateArgs")
                }
            }
            x if x == RENDER => {
                if let Some(&args) = any.downcast_ref::<RenderArgs>() {
                    Some(Event::Render(args))
                } else {
                    panic!("Expected RenderArgs")
                }
            }
            x if x == AFTER_RENDER => {
                if let Some(&args) = any.downcast_ref::<AfterRenderArgs>() {
                    Some(Event::AfterRender(args))
                } else {
                    panic!("Expected AfterRenderArgs")
                }
            }
            x if x == IDLE => {
                if let Some(&args) = any.downcast_ref::<IdleArgs>() {
                    Some(Event::Idle(args))
                } else {
                    panic!("Expected IdleArgs")
                }
            }
            _ => {
                if let &Event::Input(ref old_input) = old_event {
                    let input: Option<I> =
                        GenericEvent::from_args(event_id, any, old_input);
                    match input {
                        Some(x) => Some(Event::Input(x)),
                        None => None
                    }
                } else { return None; }
            }
        }
    }
}
