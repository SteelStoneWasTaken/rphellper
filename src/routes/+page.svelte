<script>
  import { invoke } from "@tauri-apps/api/core";
  let token = "";
  let status = "Offline";

  async function start() {
    await invoke("start", { token });
  }
  async function say() {
    await invoke("say");
  }
  async function stop() {
    await invoke("stop");
  }
  
</script>
<main class="container">
  <div class="titlebar">

  </div>
  <div class="content">
    <div class="itens"></div>
  </div>
  <div class="sidebar">
    <div class="bot">
      <input type="password" class="token" placeholder="Token" bind:value={token}>
      <button type="submit"  class="start"  on:click={start}> Run! </button> <br>
      <div class="signal"></div> {status}
    </div>
    
  </div>
</main>

<style>
  :global(body) {
    margin: 0;
  }
  main {
    background-color: #222;
    width: 99%; height: 99vh;
    
    border: solid #333;
    border-width: 0.5px;
    border-radius: 10px;

    color: aliceblue;
    overflow: hidden;
  }
  .titlebar{
    width: 100%; height: 60px;
  }
  .content{
    display: inline-block;
    width: calc(100% - 305px); height: calc(100% - 60px);
  }
  .sidebar{
    display: inline-block;
    width: 300px; height: calc(100% - 60px);

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
    width: calc(300px - 6px * 4 - 80px);
    height: 25px;
    font-size: 14px;
    background-color: #111;
    padding-left: 6px;

    border: solid #333;
    border-width: 0.5px;
    border-radius: 50px;

    color: aliceblue;
  }
  .start{
    width: 80px; height: 30px;
    font-size: 14px;
    background-color: #333;
    border: solid #444;
    border-width: 0.5px;
    border-radius: 50px;
    color: aliceblue;
  }
  .signal{
    display: inline-block;
    width: 10px; height: 10px;
    background-color: black;
    margin-left: 2px;

    border: solid #444;
    border-width: 0.5px;
    border-radius: 50px;
  }
</style>