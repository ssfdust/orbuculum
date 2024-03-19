extern crate orbuculum_grpc;
extern crate orbuculum_nm;
use eyre::{eyre, Result};
use orbuculum_grpc::{create_server, initialize_network_manager};
use orbuculum_nm::{create_channel, gather_link_modes, run_network_manager_loop, State};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::{fs, thread};
use structopt::StructOpt;

const SYS_DMI_DIR: &str = "/sys/devices/virtual/dmi/id";

#[derive(Debug, StructOpt)]
#[structopt(name = "orbuculum", about = "Usage information for orbuculum.")]
struct Argument {
    #[structopt(short, long)]
    no_initialize: bool,
    #[structopt(default_value = "127.0.0.1:15051")]
    bind_address: String,
    #[structopt(short, long, parse(from_os_str), default_value = "/etc/orbuculum")]
    config_dir: PathBuf,
}

fn format_product(info: String) -> String {
    info.trim().to_lowercase().replace(" ", "_").to_owned()
}

fn get_bios_name(dmi_dir: &Path) -> Result<String> {
    let product_name = fs::read_to_string(dmi_dir.join("product_name"))
        .map_err(|_| eyre!("Failed to read product_name"))?;
    let product_version = fs::read_to_string(dmi_dir.join("product_version"))
        .map_err(|_| eyre!("Failed to read product_version"))?;
    let version = format_product(product_version);
    let bios_name = if version.is_empty() {
        format_product(product_name)
    } else {
        format!("{}-{}", format_product(product_name), version)
    };
    Ok(bios_name)
}

/// Search the target directory defined by config_dir.
/// If the the filename matches the bios name, which could be read from
/// `/sys/devices/virtual/dmi/id/product_name` and `/sys/devices/virtual/dmi/id/product_version`
/// and the filename ends with `.rules`, return the filename.
///
/// If the `/sys/devices/virtual/dmi/id/product_version` is empty, we should just
/// use product_name as the bios name.
///
/// If none of the files match the bios name, return `default.rules`.
fn lookup_config_path(config_dir: PathBuf, dmi_dir: &Path) -> Result<String> {
    // Read the BIOS name using the provided function
    let bios_name = get_bios_name(dmi_dir)?;

    // Iterate over the files in the configuration directory
    for entry in fs::read_dir(&config_dir)? {
        let path = entry?.path();

        // Check if the file name matches the BIOS name and has a .rules extension
        if let Some(file_name) = path.file_name().and_then(|name| name.to_str()) {
            if file_name == format!("{}.rules", bios_name)
                && path.extension().and_then(|ext| ext.to_str()) == Some("rules")
            {
                return Ok(path.to_string_lossy().to_string());
            }
        }
    }

    // If no matching file was found, return the default.nic file
    Ok(config_dir
        .join("default.rules")
        .to_string_lossy()
        .to_string())
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let args = Argument::from_args();

    let (glib_sender, glib_receiver) = create_channel();
    let link_modes = gather_link_modes(None).await.unwrap();
    let arc_link_modes = Arc::new(link_modes);

    thread::spawn(move || {
        run_network_manager_loop(glib_receiver, arc_link_modes);
    });

    let shared_state = Arc::new(State::new(glib_sender));

    if !args.no_initialize {
        let config_path = lookup_config_path(args.config_dir, &PathBuf::from(SYS_DMI_DIR)).unwrap();
        initialize_network_manager(shared_state.clone(), config_path).await;
    }
    create_server(shared_state, args.bind_address)
        .await
        .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
    use std::fs;
    use tempfile::tempdir;

    #[rstest]
    #[case(
        "Product Name",
        "Product Version",
        "config/product_name-product_version.rules"
    )]
    #[case("ProductName", "", "config/productname.rules")]
    #[case(
        "ProductName",
        "Product Version",
        "config/productname-product_version.rules"
    )]
    #[case("", "NonExistent", "config/default.rules")]
    fn test_lookup_config_path(
        #[case] product_name: &str,
        #[case] product_version: &str,
        #[case] expected_result: &str,
    ) {
        // Create a temporary directory for testing
        let temp_dir = tempdir().unwrap();
        let config_dir = temp_dir.path().join("config");
        fs::create_dir_all(&config_dir).unwrap();

        // Create a temporary DMI directory with sample product_name and product_version files
        let dmi_dir = tempdir().unwrap();
        fs::write(dmi_dir.path().join("product_name"), product_name).unwrap();
        fs::write(dmi_dir.path().join("product_version"), product_version).unwrap();

        // Create some sample config files
        fs::write(config_dir.join("default.rules"), "").unwrap();
        fs::write(temp_dir.path().join(expected_result), "").unwrap();

        // Test the lookup_config_path function
        let result = lookup_config_path(config_dir.clone(), dmi_dir.path()).unwrap();
        assert_eq!(PathBuf::from(result), temp_dir.path().join(expected_result));
    }
}
