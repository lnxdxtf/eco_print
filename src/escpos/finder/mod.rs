pub mod usb {
    use rusb::{devices, DeviceList, GlobalContext};

    pub struct FinderUSB;

    impl FinderUSB {
        pub fn devices() -> Result<DeviceList<GlobalContext>, rusb::Error> {
            devices()
        }
    }
}

pub mod ble {
    use std::error::Error;
    use std::time::Duration;

    use btleplug::api::{Central, Manager as _, Peripheral, ScanFilter};
    use btleplug::platform::{Adapter, Manager};
    use tokio::time::sleep;
    use uuid::Uuid;

    pub struct FinderBLE;
    impl FinderBLE {
        pub async fn get_adapter() -> Result<Adapter, Box<dyn Error>> {
            let manager = Manager::new().await?;
            let adapter_list = manager.adapters().await?;
            if adapter_list.is_empty() {
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
            adapter.start_scan(filter).await?;
            sleep(time).await;

            let peripherals = adapter.peripherals().await?;

            Ok(peripherals)
        }

        pub async fn connect(
            peripheral: btleplug::platform::Peripheral,
        ) -> Result<btleplug::platform::Peripheral, Box<dyn Error>> {
            let connected = peripheral.is_connected().await?;
            if !connected {
                println!(
                    "Connecting to {:?}",
                    peripheral.properties().await?.unwrap().local_name
                );
                peripheral.connect().await?;
            }
            Ok(peripheral)
        }
    }
}
