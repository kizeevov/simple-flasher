use crate::core::usb_watcher;

use futures::channel::mpsc::Receiver;
use futures::StreamExt;
use iced_futures::futures;

use iced_native::subscription::{self, Subscription};

pub fn listener() -> Subscription<Event> {
    struct SomeWorker;

    subscription::unfold(
        std::any::TypeId::of::<SomeWorker>(),
        State::ListenerStarting,
        |state| async move {
            match state {
                State::ListenerStarting => {
                    let subscription = usb_watcher::subscribe();
                    (Some(Event::StartListener), State::Listener(subscription))
                }
                State::Listener(mut subscription) => {
                    let event = subscription.select_next_some().await;
                    //let event = subscription.rx_event.next();
                    println!("Event {:?}", event);

                    // match event {
                    //     Event::Initial(d) => println!("Initial devices: {:?}", d),
                    //     Event::Connect(d) => println!("Connected device: {:?}", d),
                    //     Event::Disconnect(d) => println!("Disconnected device: {:?}", d),
                    // }

                    (Some(Event::Listener), State::Listener(subscription))
                }
            }
        },
    )
}

#[derive(Debug, Clone)]
pub enum Event {
    StartListener,
    Listener,
}

enum State {
    ListenerStarting,
    Listener(Receiver<usb_enumeration::Event>),
}
