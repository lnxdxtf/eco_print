# eco_print

The eco_print library is a Rust-developed tool designed to facilitate the
connection and execution of commands in thermal printers. With this library,
users can connect to thermal printers via both Bluetooth and USB, offering
flexibility and convenience for various use cases.

Currently, eco_print only supports the Windows operating system, making it an
ideal choice for developers and users in this environment. The library was
created with the goal of integrating with a User Interface (UI), where users can
connect their device and gain full control over the printer's functionalities.

# ESCPOS

One of the main highlights of eco_print is its ability to intuitively and simply
translate ESC/POS commands, making programming for thermal printers more
accessible.

## Status

| Feature | Status |
| ------- | ------ |
| Android | 🚧     |
| IOS     | ❌     |
| Image   | 🚧     |
| QrCode  | 🚧     |
| BarCode | 🚧     |
| LOG     | ✅     |
| USB     | ✅     |
| BLE     | ✅     |

NOTE: Mobile only supports BLE

## TODO:

- [~] BLE: implement FinderBLE::scan_stream by events. To dont wait for secs.
- [~] 

## Examples:

- <a href="/examples/ble.rs">ble</a>
- <a href="/examples/escpos_ble.rs">escpos_ble</a>
- <a href="/examples/usb.rs">usb</a>
- <a href="/examples/escpos_usb.rs">escpos_usb</a>

## Tauri:

- <a href="https://github.com/lnxdxtf/tauri-plugin-escpos">tauri-plugin-escpos</a>
