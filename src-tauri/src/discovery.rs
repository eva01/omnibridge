use serde::{Deserialize, Serialize};
use serialport::{SerialPortInfo, SerialPortType};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeviceClass {
    Arduino,
    FtdiAdapter,
    Ch340,
    Cp210x,
    UnknownUsb,
    PciSerial,
    BluetoothSerial,
    Unknown,
}

impl DeviceClass {
    pub fn label(&self) -> &'static str {
        match self {
            DeviceClass::Arduino => "Arduino",
            DeviceClass::FtdiAdapter => "FTDI Adapter",
            DeviceClass::Ch340 => "CH340/CH341",
            DeviceClass::Cp210x => "CP210x",
            DeviceClass::UnknownUsb => "USB Serial",
            DeviceClass::PciSerial => "PCI Serial",
            DeviceClass::BluetoothSerial => "Bluetooth",
            DeviceClass::Unknown => "Unknown",
        }
    }

    pub fn suggested_baud(&self) -> u32 {
        9600
    }
}

fn classify_usb(vid: u16, _pid: u16) -> DeviceClass {
    match vid {
        0x2341 | 0x2A03 | 0x1B4F => DeviceClass::Arduino, // Arduino SA, Arduino LLC, SparkFun
        0x1A86 => DeviceClass::Ch340,                     // WCH CH340/CH341
        0x0403 => DeviceClass::FtdiAdapter,               // FTDI
        0x10C4 => DeviceClass::Cp210x,                    // Silicon Labs CP210x
        _ => DeviceClass::UnknownUsb,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredDevice {
    pub port: String,
    pub device_class: DeviceClass,
    pub label: String,
    pub manufacturer: Option<String>,
    pub product: Option<String>,
    pub vid: Option<u16>,
    pub pid: Option<u16>,
    pub serial_number: Option<String>,
    pub suggested_baud: u32,
}

pub fn list_devices() -> Vec<DiscoveredDevice> {
    serialport::available_ports()
        .unwrap_or_default()
        .iter()
        .map(classify_port)
        .collect()
}

fn classify_port(info: &SerialPortInfo) -> DiscoveredDevice {
    match &info.port_type {
        SerialPortType::UsbPort(usb) => {
            let class = classify_usb(usb.vid, usb.pid);
            let label = class.label().to_string();
            let suggested_baud = class.suggested_baud();
            DiscoveredDevice {
                port: info.port_name.clone(),
                device_class: class,
                label,
                manufacturer: usb.manufacturer.clone(),
                product: usb.product.clone(),
                vid: Some(usb.vid),
                pid: Some(usb.pid),
                serial_number: usb.serial_number.clone(),
                suggested_baud,
            }
        }
        SerialPortType::BluetoothPort => DiscoveredDevice {
            port: info.port_name.clone(),
            device_class: DeviceClass::BluetoothSerial,
            label: DeviceClass::BluetoothSerial.label().to_string(),
            manufacturer: None,
            product: None,
            vid: None,
            pid: None,
            serial_number: None,
            suggested_baud: 9600,
        },
        SerialPortType::PciPort => DiscoveredDevice {
            port: info.port_name.clone(),
            device_class: DeviceClass::PciSerial,
            label: DeviceClass::PciSerial.label().to_string(),
            manufacturer: None,
            product: Some("PCI Serial".to_string()),
            vid: None,
            pid: None,
            serial_number: None,
            suggested_baud: 9600,
        },
        _ => DiscoveredDevice {
            port: info.port_name.clone(),
            device_class: DeviceClass::Unknown,
            label: DeviceClass::Unknown.label().to_string(),
            manufacturer: None,
            product: None,
            vid: None,
            pid: None,
            serial_number: None,
            suggested_baud: 9600,
        },
    }
}
