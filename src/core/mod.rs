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

// let connect_option = ConnectOpts {
//     serial: None,//Some(String::from("/dev/tty.usbserial-110")),
//     speed: None
// };
// let mut config = Config::default();
// //config.connection.serial = Some(String::from("/dev/tty.usbserial-0001"));
// config.usb_device = vec![UsbDevice{
//     vid: 6790,
//     pid: 29987,
// }];
//
// let mut flasher = connect(&connect_option, &config).unwrap();
// let chip = flasher.chip();
// flasher.board_info();
// println!("{:?}", chip);
//}
