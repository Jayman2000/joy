use hidapi::HidApi;

mod calibration;
mod hid;

fn main() -> anyhow::Result<()> {
    let api = HidApi::new()?;

    for device in api
        .device_list()
        .filter(|x| x.vendor_id() == joycon_sys::NINTENDO_VENDOR_ID)
    {
        let mut device = hid::JoyCon::new(device.open_device(&api)?, device.clone());
        println!("new dev {:?}", device);
        println!("info: {:?}", device.get_dev_info()?);

        device.enable_imu()?;
        device.set_standard_mode()?;
        device.set_player_light(joycon_sys::output::PlayerLights::new(
            true, false, false, true, false, false, false, false,
        ))?;

        device.reset_calibration()?;

        for _ in 0..10 {
            let report = device.get_calibrated_gyro()?;
            println!("{:?}", report);
        }
    }
    Ok(())
}
