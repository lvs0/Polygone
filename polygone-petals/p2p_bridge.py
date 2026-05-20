import subprocess
import os
import json
from pathlib import Path

class PolygoneBridge:
    """Python bridge to the Rust Polygone core binary."""
    
    def __init__(self, binary_path: str = "../Polygone/target/debug/polygone"):
        self.binary_path = Path(binary_path).resolve()
        if not self.binary_path.exists():
            raise FileNotFoundError(f"Polygone binary not found at {self.binary_path}")

    def run_command(self, cmd_args: list):
        """Execute a polygone command and return output."""
        full_args = [str(self.binary_path)] + cmd_args
        process = subprocess.run(
            full_args,
            capture_output=True,
            text=True
        )
        return process.stdout, process.stderr

    def start_node(self, listen: str = "/ip4/0.0.0.0/tcp/4001"):
        """Spawn a background relay node."""
        print(f"Starting Polygone Relay Node on {listen}...")
        return subprocess.Popen(
            [str(self.binary_path), "node", "start", "--listen", listen],
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE
        )

    def send_message(self, peer_pk: str, message: str):
        """Send a message through the ephemeral network."""
        stdout, stderr = self.run_command(["send", "--peer-pk", peer_pk, "--message", message])
        return stdout

if __name__ == "__main__":
    # Smoke test
    try:
        bridge = PolygoneBridge()
        print("Polygone Bridge initialized.")
        out, _ = bridge.run_command(["--version"])
        print(f"Binary version: {out.strip()}")
    except Exception as e:
        print(f"Bridge test failed: {e}")
