# Pied Piper Sculpture <!-- omit in toc -->

Files used to control raspberry pi in sculpture

- [1. Hardware Setup](#1-hardware-setup)
- [2. Raspberry Pi Setup](#2-raspberry-pi-setup)
  - [2.1. Bluetooth Speaker or Other Default Audio Device](#21-bluetooth-speaker-or-other-default-audio-device)
    - [2.1.1. Detailed Bluetooth Setup](#211-detailed-bluetooth-setup)
    - [2.1.2. Bluetooth `make_autoconnect.py`](#212-bluetooth-make_autoconnectpy)
  - [2.2. Install the ALSA Audio Development Kit](#22-install-the-alsa-audio-development-kit)
  - [2.3. `raspi-config`](#23-raspi-config)
  - [2.4. Building and Installing The Code in this Repo](#24-building-and-installing-the-code-in-this-repo)
  - [2.5. Install using `systemd` to make it go on startup](#25-install-using-systemd-to-make-it-go-on-startup)
- [3. Wifi Setup](#3-wifi-setup)



## 1. Hardware Setup

The following shows the intended wiring for this project.

![wiring diagram](/readme_extras/Wiring%20Diagram.svg)

## 2. Raspberry Pi Setup

The following setup assumes a Raspberry Pi 5

### 2.1. Bluetooth Speaker or Other Default Audio Device

We need to get the PI to connect automatically with some audio device at startup.

The easiest option I have found was a bluetooth speaker. Note the following:

- The device I used will connect and play music while plugged into a charger
- To keep it from going to sleep, the program in this repo will play a one
  second low-volume humming sound once every 120 seconds to keep the speaker
  awake.
- The one downside to this setup is that if the pi is power-cycled, the
  bluetooth speaker may not attempt to reconnect. See further below instructions
  for ` bluetooth/make_auto_connect.py` which is designed to help with this
  issue.

#### 2.1.1. Detailed Bluetooth Setup
To get Bluetooth set up, we will install pulse audio and the bluetooth module:

```bash
sudo apt-get install pulseaudio pulseaudio-module-bluetooth`
```

- May need `sudo reboot` to take effect
- Pair with device (See guide
  https://gist.github.com/actuino/9548329d1bba6663a63886067af5e4cb)
  - `bluetoothctl`
  - `power on`
  - `agent on`
  - `scan on`
    - Wait for the device to be discovered
    - Note it's address; the address should look like `4E:E9:B4:0A:0F:1F`
  - `pair <device>`
    - Replace `<device>` with the address noted above
    - **you only need to type the first few characters!**, then press tab to
      auto-complete the rest.
  - `trust <device>`
    - Enables auto-connection
  - `connect <device>`
    - This should happen automatically on startup in future

#### 2.1.2. Bluetooth `make_autoconnect.py`

To get bluetooth to connect even more reliably there is a python script that
sets up a startup service that tries to force reconnection every 10 seconds if
the connection is down.

To set up this service use the command:

```bash
sudo python bluetooth/make_auto_connect.py --device 4E:E9:B4:0A:0F:1F
```

There is an `--uninstall` and `--log` command line option to remove or check the
logs for this service.


### 2.2. Install the ALSA Audio Development Kit

Install the ALSA (Advanced Linux Sound Architecture) development kit which is
needed to play music using the rust `rodio` crate.

```bash
sudo apt update
sudo apt install libasound2-dev
```

> Note: `alsa-utils` may also be needed? I don't think so...

### 2.3. `raspi-config`

```bash
sudo raspi-config
```

Enable the following options in the menu.

- `Interface Options`
  - `SPI` - Enable
  - `I2C` - Enable
- `System Options`
  - `Boot / Auto Login`
    - `Text Console, automatically logged in as...` - Enable


I2C is used to connect with the servo. SPI is needed to drive the LED lights.

The auto-login is needed to get bluetooth and audio stuff working without a user actively logged in to the device.

### 2.4. Building and Installing The Code in this Repo

4. Install git

```bash
sudo apt-get install git
```

5. clone this repository, and change your terminal's current working directory
   into the repo:

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

### 2.5. Install using `systemd` to make it go on startup

8. Install this project so that it runs on startup

```bash
sudo python service_install.py
```

9. If something goes wrong, there is are scripts to check the logs of the
   running service, and a script to uninstall it:

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

## 3. Wifi Setup

For deployment it is likely useful if the pi can connect to a hotspot so that it
can be serviced once deployed.

```bash
sudo wpa_cli scan
sudo wpa_cli scan_results
```

Tell the agent to look for available networks

```bash
sudo nmcli device wifi rescan
```

List results, connected and available networks;

```bash
nmcli device wifi list
```

connect to a new network;

```bash
 sudo nmcli device wifi connect "my-android-phone-hotspot" password "my-hotspot-password"
```
