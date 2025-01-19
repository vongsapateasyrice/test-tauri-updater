import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";

function renderApp() {
  ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <React.StrictMode>
      <App />
    </React.StrictMode>
  );
}

async function runApp() {
  renderApp();
}

runApp();
