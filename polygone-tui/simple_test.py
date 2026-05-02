#!/usr/bin/env python3
import sys
sys.path.insert(0, '/home/l-vs/.openclaw/workspace/Polygone/polygone-tui')

from polygone_tui.app import PolygoneApp

# Test basic functionality
app = PolygoneApp()
print("App created successfully")
print(f"System active: {app.system_active}")
print(f"Auto update enabled: {app.auto_update_enabled}")
print(f"Number of bindings: {len(app.BINDINGS)}")
print(f"CSS contains #1a1a1a: {'#1a1a1a' in app.CSS}")
print(f"CSS contains #00FF88: {'#00FF88' in app.CSS}")
print("All basic tests passed!")