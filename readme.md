# Pied Piper Sculpture <!-- omit in toc -->

Files used to control raspberry pi in sculpture

- [1. Hardware Setup](#1-hardware-setup)
- [2. Raspberry Pi Setup](#2-raspberry-pi-setup)
  - [2.1. Bluetooth Speaker or Other Default Audio Device](#21-bluetooth-speaker-or-other-default-audio-device)
  - [2.2. Software Installations](#22-software-installations)
  - [2.3. Building and Installing The Code in this Repo](#23-building-and-installing-the-code-in-this-repo)
  - [2.4. Install using `systemd` to make it go on startup](#24-install-using-systemd-to-make-it-go-on-startup)
  - [2.5. Auto-login](#25-auto-login)
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
  bluetooth speaker may not attempt to reconnect. To complete the connection,
  the bluetooth speaker also had to be power-cycled.

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
    - Note it's address; the address should look like `6E:E9:B4:0D:0F:18`
  - `pair <device>`
    - Replace `<device>` with the address noted above
    - **you only need to type the first few characters!**, then press tab to
      auto-complete the rest.
  - `trust <device>`
    - Enables auto-connection
  - `connect <device>`
    - This should happen automatically on startup in future


### 2.2. Software Installations

Install the ALSA (Advanced Linux Sound Architecture) development kit. 

1. install `alsa`

```bash
sudo apt update
sudo apt install alsa-utils
sudo apt install libasound2-dev
```

2. Enable both SPI and I2C
   - using `sudo raspi-config`
   - or manually by editing `/boot/firmware/config.txt` by adding ur
     uncommenting (removing the `#`)
     - `dtparam=i2c_arm=on` and
     - `dtparam=spi=on`
   - May need `sudo reboot` to take effect

### 2.3. Building and Installing The Code in this Repo

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

### 2.4. Install using `systemd` to make it go on startup

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


### 2.5. Auto-login

To make bluetooth and pulse audio work on power-on, we will configure auto-login:

```bash
sudo raspi-config
```

Select
- `System Options`
- `Boot / Auto Login` 
- `B2 Text Console, automatically logged in as...`


To get bluetooth to repair the connection with the speaker more robustly,

```bash
sudo nano /etc/bluetooth/main.conf
```

Find and set the lines

```conf
AlwaysPairable = true
FastConnectable = true
JustWorksRepairing = always
```

Then run
```bash
sudo systemctl restart bluetooth
```

## 3. Wifi Setup

For deployment it is likely useful if the pi can connect to a hotspot so that it
can be serviced once deployed.


```bash
sudo wpa_cli scan
sudo wpa_cli scan_results
```

```bash
# find avaliable networks
sudo nmcli device wifi rescan
# list connected and available networks
nmcli device wifi list

# connect to a new network;
 sudo nmcli device wifi connect "my-android-phone-hotspot" password "my-hotspot-password"
```