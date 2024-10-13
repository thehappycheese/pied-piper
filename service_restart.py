import subprocess
import sys
import tomllib
from pathlib import Path

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

# Build the service name
service_name = f"{sanitized_name}.service"

# Restart the service
try:
    print(f"Restarting service '{service_name}'...")
    subprocess.run(['systemctl', 'restart', service_name], check=True)
    print(f"Service '{service_name}' restarted successfully.\n")
except subprocess.CalledProcessError as e:
    print(f"Error restarting service: {e}", file=sys.stderr)
    sys.exit(1)

# Display the status of the service
try:
    print(f"Fetching status for service '{service_name}'...\n")
    subprocess.run(['systemctl', 'status', service_name])
except subprocess.CalledProcessError as e:
    print(f"Error fetching service status: {e}", file=sys.stderr)
    sys.exit(1)
