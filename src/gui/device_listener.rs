use crate::core::flasher::try_get_board_info;
use crate::core::usb_watcher;
use futures::channel::mpsc::Receiver;
use futures::StreamExt;
use iced_native::subscription::{self, Subscription};
use usb_enumeration::{Event as UsbEvent, UsbDevice};

pub fn listener() -> Subscription<Event> {
    struct SomeWorker;

    subscription::unfold(
        std::any::TypeId::of::<SomeWorker>(),
        State::ListenerStarting,
        |state| async move {
            match state {
                State::ListenerStarting => {
                    let subscription = usb_watcher::subscribe();
                    (None, State::Listener(subscription))
                }
                State::Listener(mut subscription) => {
                    let event = subscription.select_next_some().await;
                    match event {
                        UsbEvent::Initial(devices) => {
                            match devices
                                .iter()
                                .find(|device| try_get_board_info(device).is_ok())
                            {
                                None => (None, State::Listener(subscription)),
                                Some(device) => (
                                    Some(Event::Connect(device.clone())),
                                    State::Listener(subscription),
                                ),
                            }
                        }
                        UsbEvent::Connect(device) => match try_get_board_info(&device) {
                            Ok(_) => (Some(Event::Connect(device)), State::Listener(subscription)),
                            Err(_) => (None, State::Listener(subscription)),
                        },
                        UsbEvent::Disconnect(device) => (
                            Some(Event::Disconnect(device)),
                            State::Listener(subscription),
                        ),
                    }
                }
            }
        },
    )
}

#[derive(Debug, Clone)]
pub enum Event {
    Connect(UsbDevice),
    Disconnect(UsbDevice),
}

enum State {
    ListenerStarting,
    Listener(Receiver<usb_enumeration::Event>),
}
