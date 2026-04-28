import React from "react";
import ReactDOM from "react-dom/client";
import "./styles.css";

const rootElement = document.getElementById("root");

if (!rootElement) {
  throw new Error("Root element not found");
}

const root = ReactDOM.createRoot(rootElement);

async function bootstrap() {
  if (import.meta.env.MODE === "landing") {
    const { default: LandingPage } = await import("./landing/LandingPage");
    root.render(
      <React.StrictMode>
        <LandingPage />
      </React.StrictMode>,
    );
    return;
  }

  const { default: App } = await import("./App");
  root.render(
    <React.StrictMode>
      <App />
    </React.StrictMode>,
  );
}

bootstrap();
