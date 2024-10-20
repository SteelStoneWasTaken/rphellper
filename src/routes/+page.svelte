<script>
  import { getCurrentWindow } from '@tauri-apps/api/window';
  const appWindow = getCurrentWindow();
  
  function maximize() { appWindow.toggleMaximize() }
  function minimize() { appWindow.minimize() }
  function close()    { appWindow.close() }

  import { invoke } from "@tauri-apps/api/core"
  let token = ""
  let status = ""
  let hint = ""
  let button = ""

  async function start() {
    if (button == "Start!") {
      button = ""
      hint = ""
      status = "Starting..."
      await invoke("start", { token });
    } else if (button == "Stop!") {
      button = ""
      hint = ""
      status = "Stopping..."
    }
    
  }

  setInterval(async () => {
    status = await invoke("check");
    hint = await invoke("check_hint");

    if (status == "Online.") {
      button = "Stop!"
    } else {
      button = "Start!"
    }
  },2000);

  function say() {
    invoke("say");
  }
</script>

<main class="container">

  <!-- Titlebar -->
  <div data-tauri-drag-region  class="titlebar">
    <input type="button" class="btn mn" on:click={minimize}>
    <input type="button" class="btn mx" on:click={maximize}>
    <input type="button" class="btn cl" on:click={close}>

  </div>
  <!-- Body -->
  <input type="password" class="token" placeholder="Token" bind:value={token}>
  <button class="start"  on:click={start}> {button} </button> <br>
  {status} <b class="hint">{hint}</b> <br>
  <button type="submit"  class="say"  on:click={say}> Say! </button>

</main>

<style>
  :global(body) {
    margin: 0;
    font-family:'Franklin Gothic Medium', 'Arial Narrow', Arial, sans-serif;
    color: red;
  }
  main {
    background-color: #222;
    width: calc(100vw - 4px); height: calc(100vh - 4px);

    border: #777 solid 1px;
    border-radius: 8px;

    color: aliceblue;
  }
  .titlebar{
    text-align: right;
  }
  .btn {
    background-color: #777;
    width: 20px; height: 20px;
    margin-top: 10px;
    margin-right: 10px;

    border: none;

    border-radius: 100px;
  }
  .token {
    height: 0px; width: 150px;
    padding-top: 13px; padding-bottom: 13px;
    padding-left: 5px;
    font-size: 15px;
  }
  .start{
    vertical-align: top;
    width: 50px; height: 30px;
  }
  .hint {
    font-size: 10px;
    color: #777;
  }
</style>