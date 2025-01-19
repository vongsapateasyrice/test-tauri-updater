import { useEffect, useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  const [num1, setNum1] = useState(0);
  const [num2, setNum2] = useState(0);
  const [numSum, setNumSum] = useState(0);

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg(await invoke("greet", { name }));
  }

  async function getSum() {
    const a = await invoke("add_number", { num1, num2 });

    console.log(a);
    setNumSum(a as number);
  }

  useEffect(() => {
    getSum();
  }, [num1, num2]);

  return (
    <main className="container">
      <h1>Version 1.0.1</h1>

      <div className="row">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <p>Click on the Tauri, Vite, and React logos to learn more.</p>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>
      <p>{greetMsg}</p>
      <input
        type="number"
        onChange={(e) => {
          setNum1(Number(e.target.value));
        }}
      ></input>
      <input
        type="number"
        onChange={(e) => {
          setNum2(Number(e.target.value));
        }}
      ></input>
      <div>{num1}</div>
      <div>{num2}</div>
      <div>{numSum}</div>
    </main>
  );
}

export default App;
