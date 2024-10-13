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

# Fetch recent logs from the service
try:
    print(f"Fetching recent logs for service '{service_name}'...\n")
    # Use journalctl to get the logs
    subprocess.run(['journalctl', '-u', service_name, '--no-pager', '-n', '400'])
except subprocess.CalledProcessError as e:
    print(f"Error fetching logs: {e}", file=sys.stderr)
    sys.exit(1)
