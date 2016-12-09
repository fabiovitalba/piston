use { Event, GenericEvent, Input };

/// Update arguments, such as delta time in seconds
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct UpdateArgs {
    /// Delta time in seconds.
    pub dt: f64,
}

/// When the application state should be updated
pub trait UpdateEvent: Sized {
    /// Creates an update event.
    fn from_update_args(args: &UpdateArgs, old_event: &Self) -> Option<Self>;
    /// Creates an update event with delta time.
    fn from_dt(dt: f64, old_event: &Self) -> Option<Self> {
        UpdateEvent::from_update_args(&UpdateArgs { dt: dt }, old_event)
    }
    /// Calls closure if this is an update event.
    fn update<U, F>(&self, f: F) -> Option<U>
        where F: FnMut(&UpdateArgs) -> U;
    /// Returns update arguments.
    fn update_args(&self) -> Option<UpdateArgs> {
        self.update(|args| args.clone())
    }
}

/* TODO: Enable when specialization gets stable.
impl<T> UpdateEvent for T where T: GenericEvent {
    fn from_update_args(args: &UpdateArgs, old_event: &Self) -> Option<Self> {
        GenericEvent::from_args(UPDATE, args as &Any, old_event)
    }

    fn update<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(&UpdateArgs) -> U
    {
        if self.event_id() != UPDATE {
            return None;
        }
        self.with_args(|any| {
            if let Some(args) = any.downcast_ref::<UpdateArgs>() {
                Some(f(args))
            } else {
                panic!("Expected UpdateArgs")
            }
        })
    }
}
*/

impl UpdateEvent for Input {
    fn from_update_args(_args: &UpdateArgs, _old_event: &Self) -> Option<Self> {
        None
    }

    fn update<U, F>(&self, mut _f: F) -> Option<U>
        where F: FnMut(&UpdateArgs) -> U
    {
        None
    }
}

impl<I: GenericEvent> UpdateEvent for Event<I> {
    fn from_update_args(args: &UpdateArgs, _old_event: &Self) -> Option<Self> {
        Some(Event::Update(*args))
    }

    fn update<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(&UpdateArgs) -> U
    {
        match *self {
            Event::Update(ref args) => Some(f(args)),
            _ => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_update() {
        use Event;
        use UpdateArgs;

        let e = Event::Update(UpdateArgs { dt: 0.0 });
        let x: Option<Event> = UpdateEvent::from_update_args(
            &UpdateArgs { dt: 1.0 }, &e);
        let y: Option<Event> = x.clone().unwrap().update(|args|
            UpdateEvent::from_update_args(args, x.as_ref().unwrap())).unwrap();
        assert_eq!(x, y);
    }
}
