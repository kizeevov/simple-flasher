use espflash::cli::config::UsbDevice;
use espflash::cli::{connect, ConnectOpts};
use miette::Result;
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

    let data = try_read_firmware_file()?;
    flasher
        .load_bin_to_flash_addr(0x0, &data)
        .map_err(|_| FlashError::FlashError)?;

    Ok(())
}

fn try_read_firmware_file() -> Result<Vec<u8>, FlashError> {
    let read_dir = fs::read_dir("./").map_err(|_| FlashError::FileError)?;
    let firmware_file_path = read_dir
        .into_iter()
        .filter(|r| r.is_ok())
        .map(|r| r.unwrap().path())
        .find(|r| match r.extension() {
            None => false,
            Some(extension) => extension.eq("bin"),
        })
        .ok_or(FlashError::FileError)?;

    fs::read(firmware_file_path).map_err(|_| FlashError::FlashError)
}
