import os
import subprocess
import sys
import argparse

SERVICE_NAME = "bt_auto_connect.service"
SCRIPT_PATH = "/usr/local/bin/bt_auto_connect.sh"

def create_bash_script(device_mac):
    """Create a bash script that checks and attempts to reconnect to the Bluetooth device."""
    script_content = f"""#!/bin/bash

DEVICE="{device_mac}"

# Function to check if the device is connected
check_connection() {{
    bluetoothctl info "$DEVICE" | grep "Connected: yes" > /dev/null
    return $?
}}

# Reconnect to the device if not connected
reconnect_device() {{
    echo "Attempting to reconnect to $DEVICE"
    echo -e "connect $DEVICE\\n" | bluetoothctl > /dev/null
}}

while true; do
    if ! check_connection; then
        echo "Device $DEVICE is not connected, attempting reconnection..."
        reconnect_device
    else
        echo "Device $DEVICE is already connected."
    fi
    sleep 10  # Check every 10 seconds
done
"""

    # Write the bash script to the specified path
    with open(SCRIPT_PATH, "w") as script_file:
        script_file.write(script_content)
    
    # Make the script executable
    os.chmod(SCRIPT_PATH, 0o755)

def create_systemd_service():
    """Create a systemd service to run the bash script."""
    service_content = f"""[Unit]
Description=Bluetooth Auto Reconnect Service
After=bluetooth.target

[Service]
ExecStart={SCRIPT_PATH}
Restart=always
User=root

[Install]
WantedBy=multi-user.target
"""

    service_file = f"/etc/systemd/system/{SERVICE_NAME}"

    # Write the systemd service file
    with open(service_file, "w") as service_file:
        service_file.write(service_content)

    # Reload systemd, enable and start the service
    subprocess.run(["systemctl", "daemon-reload"], check=True)
    subprocess.run(["systemctl", "enable", SERVICE_NAME], check=True)
    subprocess.run(["systemctl", "start", SERVICE_NAME], check=True)

def uninstall_service():
    """Uninstall the systemd service and remove the bash script."""
    try:
        subprocess.run(["systemctl", "stop", SERVICE_NAME], check=True)
        subprocess.run(["systemctl", "disable", SERVICE_NAME], check=True)
        os.remove(f"/etc/systemd/system/{SERVICE_NAME}")
        os.remove(SCRIPT_PATH)
        subprocess.run(["systemctl", "daemon-reload"], check=True)
        print(f"Service {SERVICE_NAME} uninstalled successfully.")
    except FileNotFoundError:
        print("Service or script not found. Perhaps it's already uninstalled?")

def show_logs():
    """Show the logs for the systemd service."""
    subprocess.run(["journalctl", "-u", SERVICE_NAME, "-f"])

def main():
    parser = argparse.ArgumentParser(description="Bluetooth Auto Connect Script")
    parser.add_argument("--device", type=str, help="Bluetooth MAC address of the device")
    parser.add_argument("--uninstall", action="store_true", help="Uninstall the auto-connect service")
    parser.add_argument("--log", action="store_true", help="Show the auto-connect service logs")

    args = parser.parse_args()

    if args.uninstall:
        uninstall_service()
    elif args.log:
        show_logs()
    elif args.device:
        print(f"Setting up auto-connect for device: {args.device}")
        create_bash_script(args.device)
        create_systemd_service()
        print(f"Auto-connect service for device {args.device} has been set up.")
    else:
        print("Please provide a valid option. Use --device to set up or --uninstall to remove.")

if __name__ == "__main__":
    if os.geteuid() != 0:
        print("This script must be run as root.")
        sys.exit(1)
    main()
