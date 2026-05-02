#!/usr/bin/env python3
"""Polygone TUI — centralise tout dans le terminal."""
import sys
import subprocess
import os


def main():
    args = sys.argv[1:]

    # Check if polygone CLI is available
    try:
        subprocess.run(["polygone", "--version"], capture_output=True, timeout=5)
    except FileNotFoundError:
        print("ERROR: 'polygone' not found in PATH.")
        print("Install Polygone first: cargo install --path . --release")
        sys.exit(1)
    except subprocess.TimeoutExpired:
        pass

    if not args:
        # Launch the TUI
        from textual.app import run
        from polygone_tui.app import PolygoneApp
        run(PolygoneApp())
    else:
        # Delegate to CLI
        result = subprocess.run(["polygone"] + args)
        sys.exit(result.returncode)


if __name__ == "__main__":
    main()
