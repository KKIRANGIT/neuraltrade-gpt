import json
import os
from http.server import BaseHTTPRequestHandler, HTTPServer


SERVICE_NAME = os.getenv("SERVICE_NAME", "reference-service")
SERVICE_PORT = int(os.getenv("SERVICE_PORT", "8080"))
REQUIRED_CONFIGS = [key.strip() for key in os.getenv("REQUIRED_CONFIGS", "").split(",") if key.strip()]


def missing_configs():
    return [key for key in REQUIRED_CONFIGS if not os.getenv(key)]


def payload_for(path):
    missing = missing_configs()
    payload = {
        "service": SERVICE_NAME,
        "status": "ok" if not missing else "degraded",
        "path": path,
        "mode": "reference-runtime",
        "configuration": {
            "required": REQUIRED_CONFIGS,
            "missing": missing
        }
    }
    if missing:
        payload["message"] = "Configuration not set for this operation."
    else:
        payload["message"] = "Operation available."
    return payload


class Handler(BaseHTTPRequestHandler):
    def do_GET(self):
        body = json.dumps(payload_for(self.path)).encode("utf-8")
        self.send_response(200)
        self.send_header("Content-Type", "application/json")
        self.send_header("Content-Length", str(len(body)))
        self.end_headers()
        self.wfile.write(body)

    def do_POST(self):
        self.do_GET()

    def log_message(self, format, *args):
        return


if __name__ == "__main__":
    server = HTTPServer(("0.0.0.0", SERVICE_PORT), Handler)
    print(f"{SERVICE_NAME} listening on 0.0.0.0:{SERVICE_PORT}")
    server.serve_forever()
