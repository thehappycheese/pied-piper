import subprocess
import sys
import tomllib
from pathlib import Path
import shutil
import os
import pwd  # <-- Import to get user information

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
        print("The service was installed in a user's home directory, not root.")
        print("The following user accounts are available on this system:")
        print(', '.join(users))
        username = input("Please type in the username: ").strip()
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

# Get the home directory of the selected user
user_info = pwd.getpwnam(current_user)
home_dir = Path(user_info.pw_dir)

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

# Define the path for the systemd unit file
unit_file_path = Path('/etc/systemd/system') / f'{sanitized_name}.service'

# Stop the service if it's running
try:
    subprocess.run(['systemctl', 'stop', f'{sanitized_name}.service'], check=True)
    print(f"Service {sanitized_name} stopped.")
except subprocess.CalledProcessError as e:
    print(f"Service {sanitized_name} may not be running or failed to stop: {e}", file=sys.stderr)

# Disable the service
try:
    subprocess.run(['systemctl', 'disable', f'{sanitized_name}.service'], check=True)
    print(f"Service {sanitized_name} disabled.")
except subprocess.CalledProcessError as e:
    print(f"Failed to disable service {sanitized_name}: {e}", file=sys.stderr)

# Remove the systemd unit file
if unit_file_path.exists():
    try:
        unit_file_path.unlink()
        print(f"Systemd unit file {unit_file_path} removed.")
    except Exception as e:
        print(f"Error removing unit file: {e}", file=sys.stderr)
        sys.exit(1)
else:
    print(f"Systemd unit file {unit_file_path} does not exist. Skipping removal.")

# Reload the systemd daemon to apply changes
try:
    subprocess.run(['systemctl', 'daemon-reload'], check=True)
    print("Systemd daemon reloaded.")
except subprocess.CalledProcessError as e:
    print(f"Error reloading systemd daemon: {e}", file=sys.stderr)
    sys.exit(1)

# Remove the installed application directory from the correct user's home directory
installed_dir = home_dir / f".{sanitized_name}_installed"

if installed_dir.exists():
    try:
        shutil.rmtree(installed_dir)
        print(f"Installed directory {installed_dir} removed.")
    except Exception as e:
        print(f"Error removing installed directory: {e}", file=sys.stderr)
        sys.exit(1)
else:
    print(f"Installed directory {installed_dir} does not exist. Skipping removal.")

print("Un-installation completed successfully.")
