
## Install using cargo install and rc.local (NOT WORKING!)

```bash
# install as a script
cargo install --path .

#test
pied-piper
```

then 

```bash
sudo nano /etc/rc.local
```

and edit it so just before the end it does

```sh
cd /home/nick/pied-piper
pied-piper
```



## Fix bluetooth and pulse audio so they connect on startup (NOT WORKING REVERTED)

See <https://forums.gentoo.org/viewtopic-t-1110600-start-0.html>

1. edit this thingo:
```bash
sudo nano /etc/pulse/system.pa
```
add/uncomment/modify on ~ line 38
```text
load-module module-native-protocol-unix auth-anonymous=1 socket=/tmp/pulse-socket
```

2. then this thingo
```bash
sudo nano /etc/pulse/client.conf
```
add/uncomment;
```text
default-server = unix:/tmp/pulse-socket
```

3. Then do
```bash
systemctl --user stop pulseaudio.socket pulseaudio.service
systemctl --user disable pulseaudio.socket pulseaudio.service
```

```bash
sudo nano /etc/systemd/system/pulseaudio.service
```

```service
[Unit]
Description=PulseAudio system-wide sound server
Before=sound.target
Requires=sound.target
After=bluetooth.service

[Service]
Type=simple
ExecStart=/usr/bin/pulseaudio --system --disallow-exit --disable-shm --no-cpu-limit
Restart=on-failure
ExecReload=/bin/kill -HUP $MAINPID

[Install]
WantedBy=multi-user.target
```


```bash
sudo systemctl daemon-reload
sudo systemctl enable pulseaudio.service
sudo systemctl start pulseaudio.service
```

## Bluetooth Auto Connector (NOT WORKIGN ROLLED BACK)

create a script

```bash
sudo nano /usr/local/bin/bt_autoconnect.sh
```

Replace the address below with your devices mac address:

```sh
#!/bin/bash
bluetoothctl <<EOF
power on
connect XX:XX:XX:XX:XX:XX
EOF
```

create a service that calls the script

```bash
sudo nano /etc/systemd/system/bt_autoconnect.service`
```

```service
[Unit]
Description=Bluetooth Auto Connect
After=bluetooth.target

[Service]
ExecStart=/usr/local/bin/bt_autoconnect.sh
Restart=on-failure

[Install]
WantedBy=multi-user.target
```
```bash
sudo systemctl daemon-reload
sudo systemctl enable bt_autoconnect.service
sudo systemctl start bt_autoconnect.service
sudo systemctl status bt_autoconnect.service
sudo journalctl -u bt_autoconnect.service
```

## Autologin Suffering

```bash
# questionable if needed:
sudo systemctl enable bluetooth
systemctl --user enable pulseaudio
pulseaudio --start
sudo apt install pulseaudio pulseaudio-module-bluetooth
```

```bash
sudo systemctl edit getty@tty1
```


replace `pi` in the text below with your username:

```ini
[Service]
ExecStart=
ExecStart=-/sbin/agetty --autologin pi --noclear %I $TERM
```