use futures::channel::mpsc;
use futures::channel::mpsc::Receiver;
use futures::SinkExt;
use iced_futures::futures;
use usb_enumeration::{Event, Observer};

pub fn subscribe() -> Receiver<Event> {
    let (mut sender, receiver) = mpsc::channel(0);

    tokio::spawn(async move {
        let subscription = Observer::new().with_poll_interval(2).subscribe();

        for event in subscription.rx_event.iter() {
            match &event {
                Event::Initial(_) => {}
                Event::Connect(device) => {}
                Event::Disconnect(_) => {}
            };

            let _ = sender.send(event).await;
        }
    });

    receiver
}
