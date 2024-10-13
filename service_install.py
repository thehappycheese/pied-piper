
"""This install script is designed to work on linux systems using systemd.
It must be run using sudo."""

import subprocess
import sys
import tomllib
from pathlib import Path
import shutil
import os
import pwd  # <-- Add this to fetch user and group information

# Ensure the script is run with sudo
if os.geteuid() != 0:
    print("This script must be run with sudo.", file=sys.stderr)
    sys.exit(1)


def input_user():
    max_attempts = 3
    attempts = 0

    while attempts < max_attempts:
        users = [user.pw_name for user in pwd.getpwall() if int(user.pw_uid) >= 1000 and 'nologin' not in user.pw_shell]
        if len(users) == 1:
            return users[0]
        print("The service must be run in your account to access audio. The root account cannot access audio.")
        print("The following user accounts are available on this system:")
        print(', '.join(users))
        username = input("Please type in your username: ").strip()
        if not username:
            print("No username entered. Exiting.")
            sys.exit(1)
        try:
            pwd.getpwnam(username)
            return username
        except KeyError:
            print(f"User '{username}' does not exist. Please try again.")
            attempts += 1
    sys.exit(1)

current_user = input_user()

# Get the home directory and user ID of the current user
user_info = pwd.getpwnam(current_user)
home_dir = Path(user_info.pw_dir)
user_uid = user_info.pw_uid  # Get the user ID (UID)

# Read Cargo.toml to get project name
cargo_toml_path = Path('Cargo.toml')
if not cargo_toml_path.exists():
    cargo_toml_path = Path('cargo.toml')  # Try lowercase
    if not cargo_toml_path.exists():
        print("Could not find Cargo.toml or cargo.toml", file=sys.stderr)
        sys.exit(1)

with open(cargo_toml_path, 'rb') as f:
    cargo_toml = tomllib.load(f)

# Get package name
package_name = cargo_toml['package']['name']
# Replace dashes and spaces with underscores
sanitized_name = package_name.replace('-', '_').replace(' ', '_')
# Build installed folder path in the user's home directory
installed_dir = home_dir / f".{sanitized_name}_installed"
installed_dir.mkdir(parents=True, exist_ok=True)

# Find the executable
target_dir = Path('target') / 'release'
executable_name = package_name

executable_path = target_dir / executable_name
if not executable_path.exists():
    print(f"Executable {executable_path} not found. Please first run `cargo build --release`", file=sys.stderr)
    sys.exit(1)

# Copy the executable to installed_dir
shutil.copy2(executable_path, installed_dir / executable_name)

# Copy 'music' folder
music_src = Path('music')
if music_src.exists() and music_src.is_dir():
    music_dst = installed_dir / 'music'
    if music_dst.exists():
        shutil.rmtree(music_dst)
    shutil.copytree(music_src, music_dst)
else:
    print("Music folder not found. Skipping.", file=sys.stderr)

# Copy 'config.json'
config_src = Path('config.json')
if config_src.exists():
    shutil.copy2(config_src, installed_dir / 'config.json')
else:
    print("config.json not found. Skipping.", file=sys.stderr)

# Generate systemd unit file
unit_file_content = f"""[Unit]
Description={package_name} service
After=network.target

[Service]
User={current_user}
Environment=XDG_RUNTIME_DIR=/run/user/{user_uid}
ExecStart={installed_dir / executable_name}
WorkingDirectory={installed_dir}
Restart=always
StandardOutput=journal
StandardError=journal

[Install]
WantedBy=multi-user.target
"""

print("Generated systemd unit file:")

# Define the path for the systemd unit file
unit_file_path = Path('/etc/systemd/system') / f'{sanitized_name}.service'

# Write the unit file content to /etc/systemd/system
try:
    with open(unit_file_path, 'w') as f:
        f.write(unit_file_content)
    print(f"Systemd unit file written to {unit_file_path}")
except Exception as e:
    print(f"Error writing unit file: {e}", file=sys.stderr)
    sys.exit(1)

# Set correct permissions for the installed directory
try:
    shutil.chown(installed_dir, user=current_user, group=current_user)
    for root, dirs, files in os.walk(installed_dir):
        for momo in dirs:
            shutil.chown(Path(root) / momo, user=current_user, group=current_user)
        for momo in files:
            shutil.chown(Path(root) / momo, user=current_user, group=current_user)
    print(f"Set correct permissions for {installed_dir}")
except Exception as e:
    print(f"Error setting permissions: {e}", file=sys.stderr)
    sys.exit(1)

# Reload the systemd daemon to recognize the new service
try:
    subprocess.run(['systemctl', 'daemon-reload'], check=True)
    print("Systemd daemon reloaded.")
except subprocess.CalledProcessError as e:
    print(f"Error reloading systemd daemon: {e}", file=sys.stderr)
    sys.exit(1)

# Enable the service to start on boot
try:
    subprocess.run(['systemctl', 'enable', f'{sanitized_name}.service'], check=True)
    print(f"Service {sanitized_name} enabled to start on boot.")
except subprocess.CalledProcessError as e:
    print(f"Error enabling service: {e}", file=sys.stderr)
    sys.exit(1)

# Start the service immediately
try:
    subprocess.run(['systemctl', 'start', f'{sanitized_name}.service'], check=True)
    print(f"Service {sanitized_name} started successfully.")
except subprocess.CalledProcessError as e:
    print(f"Error starting service: {e}", file=sys.stderr)
    sys.exit(1)
