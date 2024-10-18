<script>
  import { getCurrentWindow } from '@tauri-apps/api/window';
  const appWindow = getCurrentWindow();
  
  function minimize() {
    appWindow.minimize()
  }
  function maximize() {
    appWindow.toggleMaximize()
  }
  function close() {
    appWindow.close()
  }

  import { invoke } from "@tauri-apps/api/core"
  let token = ""
  let button = "Run!"
  let err = ""
  let status = "Offline"
  async function start() {
    updateSignalColor("yellow");

    err = ""
    status = "Starting..."
    button = "Stop!"
    err = await invoke("start", { token });

    if (err != "") {
      status = "Offline"
      button = "Run!"
      updateSignalColor("red");
    } else {
      button = "Run!"
      updateSignalColor("black");
    }
  }

  function updateSignalColor(color) {
    const signalElement = document.querySelector('.signal');
    if (signalElement) {
      signalElement.style.backgroundColor = color;
    }
  }

</script>
<main class="container">
  <div data-tauri-drag-region  class="titlebar">
    <!-- Titlebar -->
    <input type="button" class="btn mn" on:click={minimize}>
    <input type="button" class="btn mx" on:click={maximize}>
    <input type="button" class="btn cl" on:click={close}>

  </div>
  <div class="content">
    <div class="itens"></div>
  </div>
  <div data-tauri-drag-region  class="sidebar">
    <div class="bot">
      <input type="password" class="token" placeholder="Token" bind:value={token}>
      <button type="submit"  class="start"  on:click={start}> {button} </button> <br>
      <div class="signal"></div> <c>{status}</c> <b>{err}</b>
    </div>
    
  </div>
</main>

<style>
  :global(body) {
    margin: 0;
    margin-top: 3px;
    font-family:'Franklin Gothic Medium', 'Arial Narrow', Arial, sans-serif;
    overflow: hidden;
  }
  main {
    background-color: #222;
    width: calc(100vw - 2px - 3px); height: calc(100vh - 2px - 3px);

    border: solid #333;
    border-width: 0.5px;
    border-radius: 10px;

    color: aliceblue;
    overflow: hidden;
  }
  .titlebar{
    width: 100%; height: 30px;
    text-align: right;
  }
  .btn {
    margin-top: 10px;
    margin-right: 10px;
    width: 22px; height: 22px;
    background-color: #111;

    border: solid #333;
    border-width: 0.5px;
    border-radius: 50px;

    transition: 0.3s;
  }
  .btn:hover {
    background-color: #333;
  }
  .cl:hover{
    background-color: brown;
  }
  .content{
    display: inline-block;
    width: calc(100% - 305px); height: calc(100% - 30px);
  }
  .sidebar{
    display: inline-block;
    width: 300px; height: calc(100% - 30px);

    vertical-align: top;
  }
  .itens{
    position: relative;
    left: 50%; top: 50%;
    transform: translate(-50%, -50%);
    width: calc(100% - 20px); height: calc(99% - 20px);
    background-color: #111;

    border: solid #333;
    border-width: 0.5px;
    border-radius: 10px;
  }
  .bot{
    height: 50px;
    margin-top: 15px;
  }
  .token{
    width: calc(300px - 10px * 2 - 90px);
    height: 25px;
    font-size: 14px;
    background-color: #111;
    padding-left: 10px;

    border: solid #333;
    border-width: 0.5px;
    border-radius: 50px;

    color: aliceblue;

    transition: 0.3s;
  }
  .start{
    width: 80px; height: 30px;
    font-size: 14px;
    background-color: #333;
    border: solid #444;
    border-width: 0.5px;
    border-radius: 50px;
    color: aliceblue;
    transition: 0.3s;
  }
  .start:hover {
    background-color: #444;
  }
  .signal{
    display: inline-block;
    min-width: 10px; min-height: 10px;
    
    background-color: black;

    border: solid #444;
    border-width: 0.5px;
    border-radius: 50px;
    transition: 0.3s;
  }
  b {
    position: relative;
    top: -2px;

    color: #555;
    font-size: 10px;
    vertical-align: bottom;
  }
  c {
    color: #999;
  }
</style>