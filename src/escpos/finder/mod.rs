pub use tokio;
pub use uuid;

#[cfg(feature = "usb")]
pub mod usb {
    use rusb::{devices, DeviceList, GlobalContext};

    pub struct FinderUSB;

    impl FinderUSB {
        pub fn devices() -> Result<DeviceList<GlobalContext>, rusb::Error> {
            let devices = devices();
            #[cfg(feature = "debug")]
            log::debug("eco_print::FinderUSB::devices() -> devices: {:?}", devices);
            devices
        }
    }
}

#[cfg(feature = "ble")]
pub mod ble {
    pub use btleplug;
    use std::error::Error;
    use std::time::Duration;

    use btleplug::api::{Central, Manager as _, Peripheral, ScanFilter};
    use btleplug::platform::{Adapter, Manager};
    use tokio::time::sleep;
    use uuid::Uuid;

    pub struct FinderBLE;
    impl FinderBLE {
        pub async fn get_adapter() -> Result<Adapter, Box<dyn Error>> {
            #[cfg(feature = "debug")]
            log::debug!("eco_print::FinderBLE::get_adapter() -> start");
            let manager = Manager::new().await?;
            let adapter_list = manager.adapters().await?;
            if adapter_list.is_empty() {
                #[cfg(feature = "debug")]
                log::error!("eco_print::FinderBLE::get_adapter() -> No Bluetooth adapters found");
                return Err("No Bluetooth adapters found".into());
            }
            Ok(adapter_list
                .into_iter()
                .nth(0)
                .expect("No Bluetooth adapters found"))
        }

        pub async fn scan(
            adapter: &Adapter,
            services_filter: Vec<Uuid>,
            time: Duration,
        ) -> Result<Vec<btleplug::platform::Peripheral>, Box<dyn Error>> {
            let filter = ScanFilter {
                services: services_filter,
            };
            #[cfg(feature = "debug")]
            log::debug!(
                "eco_print::FinderBLE::scan() -> start with duration: {:?} and filter: {:?}",
                time,
                filter
            );
            adapter.start_scan(filter).await?;
            sleep(time).await;

            let peripherals = adapter.peripherals().await?;
            #[cfg(feature = "debug")]
            log::debug!(
                "eco_print::FinderBLE::scan() -> peripherals found: {:?}",
                peripherals
            );

            Ok(peripherals)
        }

        pub async fn connect(
            peripheral: btleplug::platform::Peripheral,
        ) -> Result<btleplug::platform::Peripheral, Box<dyn Error>> {
            #[cfg(feature = "debug")]
            log::debug!(
                "eco_print::FinderBLE::connect() -> start with peripheral: {:?}",
                peripheral
            );
            let connected = peripheral.is_connected().await?;
            #[cfg(feature = "debug")]
            log::debug!(
                "eco_print::FinderBLE::connect() -> is_connected: {:?}",
                connected
            );
            if !connected {
                #[cfg(feature = "debug")]
                log::debug!(
                    "Connecting to {:?}",
                    peripheral.properties().await?.unwrap().local_name
                );
                peripheral.connect().await?;
            }
            Ok(peripheral)
        }
    }
}
