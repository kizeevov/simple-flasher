// use espflash::cli::{connect, ConnectOpts};
// use espflash::cli::config::UsbDevice;
// use espflash::Config;
// use usb_enumeration::{Event, Observer};

//pub fn flash() {
// let devices = usb_enumeration::enumerate(None, None);
//
// println!("{:#?}", devices);

// let sub = Observer::new()
//     .with_poll_interval(1)
//     // .with_vendor_id(0xea60)
//     // .with_product_id(0x10c4)
//     .subscribe();
//
// for event in sub.rx_event.iter() {
//     match event {
//         Event::Initial(d) => println!("Initial devices: {:?}", d),
//         Event::Connect(d) => println!("Connected device: {:?}", d),
//         Event::Disconnect(d) => println!("Disconnected device: {:?}", d),
//     }
// }
pub mod flasher;
pub mod usb_watcher;
