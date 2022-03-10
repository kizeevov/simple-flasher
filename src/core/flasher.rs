use espflash::cli::config::UsbDevice;
use espflash::cli::{connect, flash_elf_image, ConnectOpts, FlashOpts};
use espflash::{Flasher, ImageFormatId};
use miette::{IntoDiagnostic, Result};
use std::fs;
use std::str::FromStr;

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

    let mut flasher = connect(&connect_option, &config)?;
    flasher.board_info()?;

    let elf_data = fs::read("target/firmware.elf").into_diagnostic()?;
    // flasher.load_elf_to_ram(&elf_data)?;

    flash_elf_image(&mut flasher, &elf_data, None, None, None)?;

    Ok(())
}
