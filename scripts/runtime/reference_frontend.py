import html
import os
from http.server import BaseHTTPRequestHandler, HTTPServer


PORT = int(os.getenv("SERVICE_PORT", "3000"))
API_BASE_URL = os.getenv("API_BASE_URL", "http://api-gateway:8080")
CONFIG_KEYS = [key.strip() for key in os.getenv("EXTERNAL_CONFIG_KEYS", "").split(",") if key.strip()]


def missing_configs():
    return [key for key in CONFIG_KEYS if not os.getenv(key)]


class Handler(BaseHTTPRequestHandler):
    def do_GET(self):
        missing = missing_configs()
        message = (
            "Configuration not set for this operation."
            if missing
            else "All optional external integrations are configured."
        )
        items = "".join(
            f"<li><strong>{html.escape(key)}</strong>: Configuration not set for this operation.</li>"
            for key in missing
        )
        page = f"""<!doctype html>
<html lang=\"en\">
  <head>
    <meta charset=\"utf-8\" />
    <meta name=\"viewport\" content=\"width=device-width,initial-scale=1\" />
    <title>NeuralTrade Runtime</title>
    <style>
      body {{
        margin: 0;
        font-family: Arial, sans-serif;
        background: #f8fafc;
        color: #0f172a;
      }}
      .wrap {{
        max-width: 960px;
        margin: 0 auto;
        padding: 32px 20px 48px;
      }}
      .panel {{
        background: white;
        border-radius: 18px;
        padding: 24px;
        box-shadow: 0 12px 40px rgba(15, 23, 42, 0.08);
        margin-bottom: 20px;
      }}
      .warn {{
        background: #fff7ed;
        border: 1px solid #fdba74;
      }}
      code {{
        background: #e2e8f0;
        padding: 2px 6px;
        border-radius: 6px;
      }}
    </style>
  </head>
  <body>
    <div class=\"wrap\">
      <div class=\"panel\">
        <h1>NeuralTrade</h1>
        <p>The compose stack is up with <code>docker compose up -d</code>.</p>
        <p>This runtime keeps the application available even when broker, Claude, Telegram, or Razorpay credentials are missing.</p>
      </div>
      <div class=\"panel warn\">
        <h2>Configuration Status</h2>
        <p>{html.escape(message)}</p>
        <p>API gateway target: <code>{html.escape(API_BASE_URL)}</code></p>
      </div>
      <div class=\"panel\">
        <h2>Config-dependent Operations</h2>
        <ul>{items or "<li>No missing external configuration keys detected.</li>"}</ul>
      </div>
    </div>
  </body>
</html>"""
        body = page.encode("utf-8")
        self.send_response(200)
        self.send_header("Content-Type", "text/html; charset=utf-8")
        self.send_header("Content-Length", str(len(body)))
        self.end_headers()
        self.wfile.write(body)

    def log_message(self, format, *args):
        return


if __name__ == "__main__":
    server = HTTPServer(("0.0.0.0", PORT), Handler)
    print(f"frontend reference runtime listening on 0.0.0.0:{PORT}")
    server.serve_forever()
