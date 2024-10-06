# Pied Piper Sculpture

Files used to control raspberry pi in sculpture


## Dependencies

- install `rust` 
- install `alsa`
- `raspi-config`
  - enable SPI
- install pulse audio bluetooth `sudo apt-get install pulseaudio pulseaudio-module-bluetooth`
  - `sudo reboot`
  - `bluetoothctl` (See guide https://gist.github.com/actuino/9548329d1bba6663a63886067af5e4cb)
    - power on
    - agent on
    - scan on
      - wait for the device to be discovered, note it's address (you can then use tab for auto-completion)
    - pair <dev>
    - trust <dev>
    - connect <dev>

## WS2812B LED Fire Effect

uses SPI0 MOSI (Chip Pin 10? Not pin 10 on header.)

Must be enabled using `raspi-config` > `interfaces` > `SPI` possibly requires a reboot to take effect.

Hard coded for 10 LEDs
