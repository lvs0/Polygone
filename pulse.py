import http.server
import socketserver
import os
import threading
import time
import urllib.request

PORT = int(os.environ.get("PORT", "8080"))
URL = os.environ.get("RENDER_URL")

class Handler(http.server.SimpleHTTPRequestHandler):
    def do_GET(self):
        self.send_response(200)
        self.end_headers()
        self.wfile.write(b"POLYGONE PULSE: OK\n")

def pinger():
    if not URL:
        print(" [!] No RENDER_URL provided, skipping self-ping.")
        return
    
    print(f" [▸] Self-pinging active for: {URL}")
    while True:
        try:
            time.sleep(600) # 10 minutes
            print(f" [·] Sending pulse to {URL}")
            urllib.request.urlopen(URL).read()
        except Exception as e:
            print(f" [!] Pulse failed: {e}")

if __name__ == "__main__":
    # Start pinger in background
    threading.Thread(target=pinger, daemon=True).start()
    
    # Start web server
    with socketserver.TCPServer(("", PORT), Handler) as httpd:
        print(f" [✓] Pulse server listening on port {PORT}")
        httpd.serve_forever()
