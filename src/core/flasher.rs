use espflash::cli::config::UsbDevice;
use espflash::cli::{connect, ConnectOpts};
use miette::{IntoDiagnostic, Result};
use std::fs;

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

#[derive(Debug, Clone)]
pub enum FlashError {
    ConnectError,
    BoardInfoError,
    FileError,
    FlashError,
    SpawnError,
}

pub async fn flash_device(device: usb_enumeration::UsbDevice) -> Result<(), FlashError> {
    tokio::task::spawn_blocking(move || flash(&device))
        .await
        .map_err(|_| FlashError::SpawnError)?
}

fn flash(device: &usb_enumeration::UsbDevice) -> Result<(), FlashError> {
    let connect_option = ConnectOpts {
        serial: None,
        speed: None,
    };
    let mut config = espflash::Config::default();
    config.usb_device = vec![UsbDevice {
        vid: device.vendor_id,
        pid: device.product_id,
    }];

    let mut flasher = connect(&connect_option, &config).map_err(|_| FlashError::ConnectError)?;
    flasher
        .board_info()
        .map_err(|_| FlashError::BoardInfoError)?;

    let data = fs::read("firmware.bin")
        .into_diagnostic()
        .map_err(|_| FlashError::FileError)?;
    flasher
        .load_bin_to_flash_addr(0x0, &data)
        .map_err(|_| FlashError::FlashError)?;

    Ok(())
}
