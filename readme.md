# Pied Piper Sculpture

Files used to control raspberry pi in sculpture

## Wiring


The following shows the intended wiring for this project.

![wiring diagram](/readme_extras/Wiring%20Diagram.svg)


## Bluetooth Speaker or Other Default Audio Device

It is expected that the PI is already configured to automatically connect with a bluetooth audio device or at least a default audio device is configured.

A 1 second low-volume humming sound is played once every 120 seconds to keep the speaker awake.

## Setup



1. install `alsa`

```bash
sudo apt update
sudo apt install alsa-utils
sudo apt install libasound2-dev
```

2. Enable both SPI and I2C
   - using `sudo raspi-config`
   - or manually by editing `/boot/firmware/config.txt` by adding ur uncommenting (removing the `#`)
     - `dtparam=i2c_arm=on` and
     - `dtparam=spi=on`
   - May need `sudo reboot` to take effect
3. **If using bluetooth**: install pulse audio bluetooth `sudo apt-get install pulseaudio pulseaudio-module-bluetooth`
  - May need `sudo reboot` to take effect
  - Pair with device (See guide https://gist.github.com/actuino/9548329d1bba6663a63886067af5e4cb)
    - `bluetoothctl`
    - `power on`
    - `agent on`
    - `scan on`
      - Wait for the device to be discovered
      - Note it's address; the address should look like `6E:E9:B4:0D:0F:18`
    - `pair <device>`
      - Replace `<device>` with the address noted above
      - **you only need to type the first few characters!**, then press tab to auto-complete the rest.
    - `trust <device>`
      - Enables auto-connection
    - `connect <device>`
      - This should happen automatically on startup in future


4. Install git

```bash
sudo apt-get install git
```

5. clone this respository, and change your terminal's current working directory into the repo:

```bash
git clone https://github.com/thehappycheese/pied-piper
cd pied-piper
```

6. Install `rust`

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

7. Build this project

```bash
cargo build --release
```

8. Install this project so that it runs on startup

```bash
sudo python service_install.py
```

9. If something goes wrong, there is are scripts to check the logs of the running service, and a script to uninstall it:

Check logs of running service:

```bash
sudo python service_check.py
```

Restart running service:

```bash
sudo python service_restart.py
```

Uninstall the service:

```bash
sudo python service_uninstall.py
```

## WS2812B LED Fire Effect

uses SPI0 MOSI (Chip Pin 10? Not pin 10 on header.)

Must be enabled using `raspi-config` > `interfaces` > `SPI` possibly requires a reboot to take effect.

Hard coded for 10 LEDs
