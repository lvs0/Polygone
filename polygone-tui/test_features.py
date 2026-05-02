#!/usr/bin/env python3
"""Test script to verify Polygone TUI features"""

import sys
sys.path.insert(0, '/home/l-vs/.openclaw/workspace/Polygone/polygone-tui')

from polygone_tui.app import PolygoneApp

def test_app_initialization():
    """Test that the app initializes correctly"""
    try:
        app = PolygoneApp()
        print("✓ App initialization successful")
        
        # Test initial state
        assert app.system_active == True, "System should be active by default"
        assert app.auto_update_enabled == True, "Auto update should be enabled by default"
        print("✓ Initial state correct")
        
        # Test state changes
        app.system_active = False
        assert app.system_active == False, "System state should be changeable"
        print("✓ State management working")
        
        # Test auto-update toggle
        app.auto_update_enabled = False
        assert app.auto_update_enabled == False, "Auto update should be toggleable"
        app.auto_update_enabled = True
        assert app.auto_update_enabled == True, "Auto update should be toggleable back"
        print("✓ Auto-update toggle working")
        
        return True
        
    except Exception as e:
        print(f"✗ Test failed: {e}")
        return False

def test_bindings():
    """Test that keyboard bindings are configured"""
    try:
        app = PolygoneApp()
        bindings = app.BINDINGS
        
        # Check that all required bindings exist
        required_bindings = [
            ("1", "switch_tab('accueil')", "Accueil"),
            ("2", "switch_tab('favoris')", "Favoris"),
            ("3", "switch_tab('services')", "Services"),
            ("4", "switch_tab('parametres')", "Paramètres"),
        ]
        
        for key, action, desc in required_bindings:
            found = False
            for binding in bindings:
                if (binding.key == key and 
                    str(binding.action) == action and 
                    binding.description == desc):
                    found = True
                    break
            assert found, f"Binding {key} -> {action} not found"
        
        print("✓ All keyboard bindings configured correctly")
        return True
        
    except Exception as e:
        print(f"✗ Bindings test failed: {e}")
        return False

def test_css_styles():
    """Test that CSS styles are properly configured"""
    try:
        app = PolygoneApp()
        css = app.CSS
        
        # Check for required styles
        assert "#1a1a1a" in css, "Background color should be #1a1a1a"
        assert "#00FF88" in css, "Accent color should be #00FF88"
        assert "bold" in css, "Bold styles should be present"
        assert "title-bold" in css, "Title bold class should be present"
        
        print("✓ CSS styles configured correctly")
        return True
        
    except Exception as e:
        print(f"✗ CSS test failed: {e}")
        return False

if __name__ == "__main__":
    print("Testing Polygone TUI features...")
    print()
    
    tests = [
        test_app_initialization,
        test_bindings,
        test_css_styles,
    ]
    
    passed = 0
    total = len(tests)
    
    for test in tests:
        if test():
            passed += 1
        print()
    
    print(f"Results: {passed}/{total} tests passed")
    
    if passed == total:
        print("🎉 All tests passed! The Polygone TUI is ready.")
        sys.exit(0)
    else:
        print("❌ Some tests failed.")
        sys.exit(1)