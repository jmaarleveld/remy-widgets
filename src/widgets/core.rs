use ratatui::widgets::{StatefulWidgetRef, WidgetRef};

pub trait RemyWidget: WidgetRef {
}

pub trait RemyWidgetCommandConverter<T> {
    type Event;

    fn convert(event: crate::events::Event, state: &T) -> Option<Self::Event>;
}


pub trait RemyWidgetState: Sized {
    type Command;
    type EventOutput;
    
    fn handle_events<T>(&mut self, event: crate::events::Event) -> Self::EventOutput
    where T: RemyWidgetCommandConverter<Self, Event=Self::Command>
    {
        self.handle_native_event(T::convert(event, self))
    }
    
    fn handle_native_event(&mut self, event: Option<Self::Command>) -> Self::EventOutput;
}

pub trait StatefulRemyWidget: StatefulWidgetRef<State=Self::Input> {
    type Input: RemyWidgetState;
}
