use espflash::cli::config::UsbDevice;
use espflash::cli::{connect, ConnectOpts};
use espflash::Flasher;
use miette::Result;

pub fn try_get_board_info(device: &usb_enumeration::UsbDevice) -> Result<()> {
    let connect_option = espflash::cli::ConnectOpts {
        serial: None,
        speed: None,
    };
    let mut config = espflash::Config::default();
    config.usb_device = vec![UsbDevice {
        vid: device.vendor_id,
        pid: device.product_id,
    }];

    connect(&connect_option, &config)?;
    Ok(())
}

pub fn flash(device: &usb_enumeration::UsbDevice) -> Result<()> {
    let connect_option = ConnectOpts {
        serial: None,
        speed: None,
    };
    let mut config = espflash::Config::default();
    config.usb_device = vec![UsbDevice {
        vid: device.vendor_id,
        pid: device.product_id,
    }];

    let flasher = connect(&connect_option, &config);

    //flash_elf_image()

    Ok(())
}
