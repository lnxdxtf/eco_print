use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use btleplug::{
    api::{
        bleuuid::uuid_from_u16, Central, CentralEvent, Manager as ManagerTrait,
        Peripheral as PeripheralTrait, ScanFilter, WriteType,
    },
    platform::{Adapter, Manager, Peripheral},
};
use futures_lite::StreamExt;
use tokio::{
    runtime::{Builder, Runtime},
    sync::Mutex,
    task,
};
use uuid::Uuid;

use crate::{EcoPrintError, EcoPrintResult, FinderTrait, PrinterTrait};

pub const THERMAL_PRINTER_SERVICE: &'static str = "000018f0-0000-1000-8000-00805f9b34fb";
pub const THERMAL_PRINTER_CHR_0: Uuid = uuid_from_u16(0x2af0);
pub const THERMAL_PRINTER_CHR_1: Uuid = uuid_from_u16(0x2af1);

pub struct ESCPOSPrinterBLE {
    // _runtime: tokio::runtime::Runtime,
    _device: Option<Peripheral>,
    _adapter: Option<Adapter>,
    devices: Arc<Mutex<Vec<Peripheral>>>,
}

impl ESCPOSPrinterBLE {
    pub fn new() -> EcoPrintResult<Self> {
        // let runtime = Self::_create_runtime()?;
        Ok(Self {
            // _runtime: runtime,
            _adapter: None,
            _device: None,
            devices: Arc::new(Mutex::new(Vec::new())),
        })
    }

    fn _create_runtime() -> EcoPrintResult<Runtime> {
        log::info!("ESCPOSPrinterBLE::_create_runtime() start");
        let runtime = {
            #[cfg(not(target_os = "android"))]
            {
                Builder::new_multi_thread()
                    .worker_threads(2)
                    .enable_all()
                    .thread_name_fn(|| {
                        static ATOMIC_ID: AtomicUsize = AtomicUsize::new(0);
                        let id = ATOMIC_ID.fetch_add(1, Ordering::SeqCst);
                        format!("tokio-runtime-worker-{}", id)
                    })
                    .build()
                    .map_err(|_err| EcoPrintError::Runtime(_err.to_string()))
            }

            #[cfg(target_os = "android")]
            {
                Builder::new_multi_thread()
                    .worker_threads(2)
                    .enable_all()
                    .thread_name_fn(|| {
                        static ATOMIC_ID: AtomicUsize = AtomicUsize::new(0);
                        let id = ATOMIC_ID.fetch_add(1, Ordering::SeqCst);
                        format!("tokio-runtime-worker-android-{}", id)
                    })
                    .build()
                    .map_err(|_err| EcoPrintError::Runtime(_err.to_string()))
            }
        };
        log::info!("ESCPOSPrinterBLE::_create_runtime() done");
        runtime
    }
}

impl PrinterTrait for ESCPOSPrinterBLE {
    async fn print(&mut self, data: String) -> EcoPrintResult<()> {
        log::info!("PrinterESCPOSBLE::print() start");

        if let Some(device) = &self._device {
            let chr = device
                .characteristics()
                .iter()
                .find(|chr| chr.uuid == THERMAL_PRINTER_CHR_1)
                .expect("Characteristicnot found")
                .clone();

            device
                .write(&chr, data.as_bytes(), WriteType::WithoutResponse)
                .await
                .map_err(|_err| {
                    EcoPrintError::Printing(format!(
                        "Peripheral characteristic write/printing - {:?}",
                        _err.to_string()
                    ))
                })?;

            log::info!("PrinterESCPOSBLE::print() done");
        } else {
            return Err(EcoPrintError::BLEPeripheral(format!(
                "Peripheral not connected"
            )));
        }
        Ok(())
    }
}

impl FinderTrait<Peripheral> for ESCPOSPrinterBLE {
    async fn start(&mut self) -> crate::EcoPrintResult<()> {
        let manager = Manager::new().await.map_err(|_err| {
            EcoPrintError::BLEAdapter(format!("Error creating manager - {:?}", _err.to_string()))
        })?;
        let adapter = manager
            .adapters()
            .await
            .map_err(|_err| {
                EcoPrintError::BLEAdapter(format!(
                    "Error getting adapters - {:?}",
                    _err.to_string()
                ))
            })?
            .into_iter()
            .nth(0)
            .unwrap();

        self._adapter = Some(adapter.clone());

        self.scan().await?;

        let mut events = adapter.events().await.map_err(|_err| {
            EcoPrintError::BLEAdapter(format!(
                "Error getting events adapter - {:?}",
                _err.to_string()
            ))
        })?;

        #[allow(unused)]
        let mut devices = Arc::clone(&self.devices);

        // spawn a new task to listen to the events
        task::spawn(async move {
            while let Some(event) = events.next().await {
                log::info!("Event: {:?}", event);
                match event {
                    CentralEvent::DeviceDiscovered(_) => {
                        let mut devices = devices.lock().await;
                        let _devices = adapter.peripherals().await.map_err(|_err| {
                            EcoPrintError::BLEScan(format!(
                                "Error on discovery devices - {:?}",
                                _err.to_string()
                            ))
                        });
                        match _devices {
                            Ok(_devices) => {
                                devices.clear();
                                for device in _devices {
                                    devices.push(device);
                                }
                            }
                            Err(_err) => {
                                log::error!("Error on discovery devices - {:?}", _err);
                            }
                        }
                    }
                    _ => {}
                }
            }
        });

        Ok(())
    }
    async fn scan(&mut self) -> crate::EcoPrintResult<()> {
        if let Some(adapter) = &self._adapter {
            adapter
                .start_scan(ScanFilter::default())
                .await
                .map_err(|_err| {
                    EcoPrintError::BLEScan(format!("Error scan - {:?}", _err.to_string()))
                })?;
        }
        Ok(())
    }
    async fn get_devices(&self) -> Vec<Peripheral> {
        let devices = self.devices.lock().await.clone();
        devices
    }
    async fn connect(&mut self, device: Peripheral) -> crate::EcoPrintResult<()> {
        if let Some(_) = &self._device {
            log::info!("Peripheral already connected");
            return Ok(());
        }

        let device_connected = device.is_connected().await.map_err(|_err| {
            EcoPrintError::BLEPeripheral(format!(
                "Error on check if device is connected - {:?}",
                _err.to_string()
            ))
        })?;
        if !device_connected {
            device.connect().await.map_err(|_err| {
                EcoPrintError::BLEPeripheral(format!(
                    "Error on connect to device - {:?}",
                    _err.to_string()
                ))
            })?;
        }
        // To check if the device is connected now
        let device_connected = device.is_connected().await.map_err(|_err| {
            EcoPrintError::BLEPeripheral(format!(
                "Error on check if device is connected - {:?}",
                _err.to_string()
            ))
        })?;
        if device_connected {
            self._device = Some(device);
        }

        Ok(())
    }

    async fn disconnect(&mut self) -> crate::EcoPrintResult<()> {
        if let None = self._device {
            log::info!("Peripheral already disconnected");
            return Ok(());
        }

        if let Some(device) = &self._device {
            device.disconnect().await.map_err(|_err| {
                EcoPrintError::BLEPeripheral(format!(
                    "Error on disconnect from device - {:?}",
                    _err.to_string()
                ))
            })?;
            self._device = None;
        }

        Ok(())
    }
}
