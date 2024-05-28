use std::time::Duration;

use serialport::{available_ports, SerialPort};

//use core::result::Result as _;
use nrfdfu::Result;

/// Nordic's vendor ID. Nordic's default nRF52 bootloader supplies this vendor ID. If the device has
/// a custom bootloader that supplies a different VID, this utility will not work.
///
/// See https://usb.org/members which lists Nordic Semiconductor's decimal ID as 6421.
const NORDIC_BOOTLOADER_USB_VID: u16 = 0x1915;

/// The product ID supplied by Nordic's default nRF52 bootloader. If the device has a custom
/// bootloader that supplies a different PID, this utility will not work.
const NORDIC_BOOTLOADER_USB_PID: u16 = 0x521f;

fn main()
{
    match main_body()
    {
        Ok(()) => {}
        Err(e) => {
            eprintln!("error: {}", e);
            std::process::exit(1);
        }
    }
}

fn main_body() -> Result<()>
{
    let zip_path = std::env::args_os()
        .nth(1)
        .ok_or_else(|| "missing argument (expected path to .zip file)".to_string())?;
    let zip_path = zip_path.to_str().unwrap();

    let port = select_port(115200)?;

    return nrfdfu::run(zip_path, port)
}

fn select_port(
    baud_rate: u32
) -> Result<Box<dyn SerialPort>>
{
    let matching_ports: Vec<_> = available_ports()?
        .into_iter()
        .filter(|port| match &port.port_type {
            serialport::SerialPortType::UsbPort(usb) =>
                usb.vid == NORDIC_BOOTLOADER_USB_VID
                    && usb.pid == NORDIC_BOOTLOADER_USB_PID,
            _ => false,
        })
        .collect();

    return match matching_ports.len() {
        0 => {
            Err(
                "no matching USB serial device found.\n\
                Remember to put the device in bootloader mode!"
                    .to_string()
                    .into()
            )
        }
        1 => {
            let port = &matching_ports[0].port_name;
            log::debug!("opening {} (type {:?})", port, matching_ports[0].port_type);
            let port = serialport::new(port, baud_rate)
                .timeout(Duration::from_millis(60000)) // TODO: accept timeout value as run param
                .open()?;
            Ok(port)
        }
        _ => Err(
            "multiple matching USB serial devices found.\n\
            This utility only works when a single device is in bootloader mode."
                .to_string()
                .into()
        ),
    };
}
