import type { Config } from "tailwindcss";

const config: Config = {
  content: [
    "./app/**/*.{ts,tsx}",
    "./components/**/*.{ts,tsx}",
    "./lib/**/*.{ts,tsx}"
  ],
  theme: {
    extend: {
      colors: {
        ink: "#0f172a",
        mist: "#f8fafc",
        pine: "#0f766e",
        ember: "#dc2626",
        sand: "#f2e8cf",
        copper: "#b45309"
      },
      boxShadow: {
        panel: "0 18px 60px rgba(15, 23, 42, 0.10)"
      },
      backgroundImage: {
        "radial-mesh":
          "radial-gradient(circle at top left, rgba(20,184,166,0.25), transparent 28%), radial-gradient(circle at top right, rgba(245,158,11,0.18), transparent 24%), radial-gradient(circle at bottom left, rgba(59,130,246,0.16), transparent 30%)"
      }
    }
  },
  plugins: []
};

export default config;
