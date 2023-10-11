use std::path::Path;
use udev::Device;

pub fn get_dev_id_path(device_syspath: Option<&str>) -> Option<String> {
    let mut dev_id_path = None;
    if let Some(device_syspath) = device_syspath {
        let device_syspath = Path::new(device_syspath);
        if let Ok(device) = Device::from_syspath(&device_syspath) {
            dev_id_path = device
                .property_value("ID_PATH")
                .map(|s| s.to_string_lossy().to_string());
        }
    }
    dev_id_path
}
