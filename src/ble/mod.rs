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

// Android
#[cfg(target_os = "android")]
use jni::AttachGuard;
#[cfg(target_os = "android")]
use std::cell::RefCell;
#[cfg(target_os = "android")]
std::thread_local! {
    static JNI_ENV: RefCell<Option<AttachGuard<'static>>> = RefCell::new(None);
}

pub const THERMAL_PRINTER_SERVICE: &'static str = "000018f0-0000-1000-8000-00805f9b34fb";
pub const THERMAL_PRINTER_CHR_0: Uuid = uuid_from_u16(0x2af0);
pub const THERMAL_PRINTER_CHR_1: Uuid = uuid_from_u16(0x2af1);

pub struct ESCPOSPrinterBLE {
    _runtime: tokio::runtime::Runtime,
    _ble_shared_data: Arc<Mutex<BLESharedData>>,
}

#[derive(Default)]
struct BLESharedData {
    _device: Option<Peripheral>,
    _adapter: Option<Adapter>,
    _devices: Vec<Peripheral>,
}

impl ESCPOSPrinterBLE {
    pub fn new() -> EcoPrintResult<Self> {
        Ok(Self {
            _runtime: Self::_create_runtime()?,
            _ble_shared_data: Arc::new(Mutex::new(BLESharedData::default())),
        })
    }

    fn _create_runtime() -> EcoPrintResult<Runtime> {
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
                use crate::android_ble::{setup_class_loader, JAVAVM};

                let vm = JAVAVM.get().ok_or(AndroidError::JavaVM)?;
                let env = vm.attach_current_thread().unwrap();
                let class_loader = setup_class_loader(&env);

                Builder::new_multi_thread()
                    .worker_threads(2)
                    .enable_all()
                    .thread_name_fn(|| {
                        static ATOMIC_ID: AtomicUsize = AtomicUsize::new(0);
                        let id = ATOMIC_ID.fetch_add(1, Ordering::SeqCst);
                        format!("tokio-runtime-worker-android-{}", id)
                    })
                    .on_thread_stop(move || {
                        log::info!("JNI thread stopped");

                        JNI_ENV.with(|f| *f.borrow_mut() = None);
                    })
                    .on_thread_start(move || {
                        log::info!("JNI thread started");

                        let vm = JAVAVM.get().unwrap();
                        let env = vm.attach_current_thread().unwrap();

                        let thread = env
                            .call_static_method(
                                "java/lang/Thread",
                                "currentThread",
                                "()Ljava/lang/Thread;",
                                &[],
                            )
                            .unwrap()
                            .l()
                            .unwrap();
                        env.call_method(
                            thread,
                            "setContextClassLoader",
                            "(Ljava/lang/ClassLoader;)V",
                            &[class_loader.as_ref().unwrap().as_obj().into()],
                        )
                        .unwrap();
                        JNI_ENV.with(|f| *f.borrow_mut() = Some(env));
                    })
                    .build()
                    .map_err(|_err| EcoPrintError::Runtime(_err.to_string()))
            }
        };
        runtime
    }
}

impl PrinterTrait for ESCPOSPrinterBLE {
    async fn print(&mut self, data: String) -> EcoPrintResult<()> {
        let runtime = &self._runtime;
        let ble_shared_data_clone = Arc::clone(&self._ble_shared_data);

        runtime
            .spawn({
                let ble_shared_data_clone = Arc::clone(&ble_shared_data_clone);
                async move {
                    if let Some(device) = &ble_shared_data_clone.lock().await._device {
                        device.discover_services().await.map_err(|_err| {
                            EcoPrintError::BLEConnect(format!(
                                "Discover services on connect - {}",
                                _err.to_string()
                            ))
                        })?;
                        let chrs = device.characteristics();

                        let chr = chrs
                            .iter()
                            .find(|chr| chr.uuid == THERMAL_PRINTER_CHR_1)
                            .expect("Characteristic not found")
                            .clone();

                        device
                            .write(&chr, data.as_bytes(), WriteType::WithoutResponse)
                            .await
                            .map_err(|_err| {
                                EcoPrintError::Printing(format!(
                                    "Peripheral characteristic write/printing - {}",
                                    _err.to_string()
                                ))
                            })?;
                    } else {
                        return Err(EcoPrintError::BLEPeripheral(format!(
                            "Peripheral not connected"
                        )));
                    }
                    Ok::<(), EcoPrintError>(())
                }
            })
            .await
            .map_err(|_err| EcoPrintError::Runtime(_err.to_string()))?
    }
}

impl FinderTrait<Peripheral> for ESCPOSPrinterBLE {
    async fn start(&mut self) -> crate::EcoPrintResult<()> {
        let runtime = &self._runtime;
        let ble_shared_data_clone = Arc::clone(&self._ble_shared_data);

        let _ = runtime
            .spawn({
                let ble_shared_data_clone = Arc::clone(&ble_shared_data_clone);
                async move {
                    let manager = Manager::new().await.map_err(|_err| {
                        EcoPrintError::BLEAdapter(format!(
                            "Error creating manager - {}",
                            _err.to_string()
                        ))
                    })?;
                    let adapter = manager
                        .adapters()
                        .await
                        .map_err(|_err| {
                            EcoPrintError::BLEAdapter(format!(
                                "Error getting adapters - {}",
                                _err.to_string()
                            ))
                        })?
                        .into_iter()
                        .nth(0)
                        .unwrap();
                    ble_shared_data_clone.lock().await._adapter = Some(adapter.clone());

                    Ok::<(), EcoPrintError>(())
                }
            })
            .await
            .map_err(|_err| EcoPrintError::Runtime(_err.to_string()))?;

        let adapter = ble_shared_data_clone.lock().await._adapter.clone().unwrap();
        let mut events = adapter.events().await.map_err(|_err| {
            EcoPrintError::BLEAdapter(format!(
                "Error getting events adapter - {}",
                _err.to_string()
            ))
        })?;

        let ble_shared_data_clone = Arc::clone(&ble_shared_data_clone);

        let _ = task::spawn(async move {
            while let Some(event) = events.next().await {
                match event {
                    CentralEvent::DeviceDiscovered(_) => {
                        let mut devices_guard = ble_shared_data_clone.lock().await;
                        devices_guard._devices.clear();
                        let _devices = adapter.peripherals().await.map_err(|_err| {
                            EcoPrintError::BLEScan(format!(
                                "Error on discovery devices - {}",
                                _err.to_string()
                            ))
                        });
                        match _devices {
                            Ok(_devices) => {
                                for device in _devices {
                                    let d_name = device
                                        .properties()
                                        .await
                                        .unwrap()
                                        .unwrap()
                                        .local_name
                                        .unwrap_or("Unknown".to_string());
                                    log::info!("Device discovered: {}", d_name);
                                    devices_guard._devices.push(device);
                                }
                            }
                            Err(_err) => {
                                log::error!("Error on discovery devices - {}", _err.to_string());
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
        let runtime = &self._runtime;
        let ble_shared_data_clone = Arc::clone(&self._ble_shared_data);
        runtime
            .spawn({
                let ble_shared_data_clone = Arc::clone(&ble_shared_data_clone);
                async move {
                    let _adapter = ble_shared_data_clone.lock().await._adapter.clone();
                    if let Some(adapter) = _adapter {
                        adapter
                            .start_scan(ScanFilter::default())
                            .await
                            .map_err(|_err| {
                                EcoPrintError::BLEScan(format!("Error scan - {}", _err.to_string()))
                            })?;
                    }
                    Ok::<(), EcoPrintError>(())
                }
            })
            .await
            .map_err(|_err| EcoPrintError::Runtime(_err.to_string()))?
    }

    async fn get_devices(&self) -> Vec<Peripheral> {
        self._ble_shared_data.lock().await._devices.clone()
    }
    async fn connect(&mut self, device: Peripheral) -> crate::EcoPrintResult<()> {
        let runtime = &self._runtime;
        let ble_shared_data_clone = Arc::clone(&self._ble_shared_data);
        runtime
            .spawn({
                let ble_shared_data_clone = Arc::clone(&ble_shared_data_clone);
                async move {
                    let mut _device_guard = ble_shared_data_clone.lock().await;
                    let mut _device = &_device_guard._device;
                    if let Some(device) = _device {
                        if device.is_connected().await.map_err(|_err| {
                            EcoPrintError::BLEPeripheral(format!(
                                "Error on check if device is connected - {}",
                                _err.to_string()
                            ))
                        })? {
                            log::info!("Peripheral already connected");
                        } else {
                            log::info!("Peripheral not connected and is setted, trying to connect");
                            device.connect().await.map_err(|_err| {
                                EcoPrintError::BLEPeripheral(format!(
                                    "Error on connect to device - {}",
                                    _err.to_string()
                                ))
                            })?;
                        }
                        return Ok::<(), EcoPrintError>(());
                    }
                    log::info!("Peripheral not connected, trying to connect");
                    device.connect().await.map_err(|_err| {
                        EcoPrintError::BLEPeripheral(format!(
                            "Error on connect to device - {}",
                            _err.to_string()
                        ))
                    })?;
                    let connected = device.is_connected().await.map_err(|_err| {
                        EcoPrintError::BLEPeripheral(format!(
                            "Error on check if device is connected - {}",
                            _err.to_string()
                        ))
                    })?;

                    if connected {
                        _device_guard._device = Some(device);
                        log::info!("Peripheral connected");
                    }
                    Ok::<(), EcoPrintError>(())
                }
            })
            .await
            .map_err(|_err| EcoPrintError::Runtime(_err.to_string()))?
    }

    async fn disconnect(&mut self) -> crate::EcoPrintResult<()> {
        let device = &self._ble_shared_data.lock().await._device;

        if let None = device {
            log::info!("Peripheral already disconnected");
            return Ok(());
        }

        if let Some(dvc) = device {
            dvc.disconnect().await.map_err(|_err| {
                EcoPrintError::BLEPeripheral(format!(
                    "Error on disconnect from device - {}",
                    _err.to_string()
                ))
            })?;
            self._ble_shared_data.lock().await._device = None;
        }

        Ok(())
    }
}
