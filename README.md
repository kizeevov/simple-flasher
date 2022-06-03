# Simple Flasher

A cross-platform GUI application that allows you to install firmware on your esp8266 with one button.
<p align="center">
  <img src="https://user-images.githubusercontent.com/29508723/160273156-dee5d6fe-754e-4971-8712-b650b932496e.png">
</p>

<p align="center">
  <img src="https://user-images.githubusercontent.com/29508723/160273124-ccea5f54-f4ee-4416-8608-b48c9e8e936f.png">
</p>

# What is it for

You have an ESP device for which you want to release new firmware versions, and you want users to be able to install on their devices without any problems

# Features

* [x] Automatic device discovery
* [ ] Installing serial driver for Windows
* [ ] Embedded firmware file
* [ ] Localization support

# Supported chips

* [x] ESP8266
* [ ] ESP32

# Building and running from source

First you must install [Rust](https://www.rust-lang.org/).

Then, clone the project and run:

`cargo build`

For a release optimized build run:

`cargo build --release`

Run the command to run the application:

`cargo run`

# Credits

[iced](https://github.com/iced-rs/iced) project for allowing me to create a user interface like this!

[espflash](https://github.com/esp-rs/espflash) project with basic esp firmware flashing methods
