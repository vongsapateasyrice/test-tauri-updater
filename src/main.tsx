import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import { check } from "@tauri-apps/plugin-updater";

function renderApp() {
  ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <React.StrictMode>
      <App />
    </React.StrictMode>
  );
}

async function checkAndUpdate() {
  const update = await check();

  console.log("hasUpdate", update?.available);
  console.log("currentver", update?.currentVersion);
  console.log("ver", update?.version);
  console.log(await update?.download());
}

async function runApp() {
  await checkAndUpdate();
  renderApp();
}

runApp();
