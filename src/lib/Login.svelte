<script lang="ts">
  import "../app.css";
  import { invoke } from "@tauri-apps/api/core";
  import { createEventDispatcher } from "svelte";
  let password = "";
  let username = "";

  async function onEnter() {
    // encoded = await invoke<string>("hash_password", { password });
    // derived = await invoke<string>("derive_key", { password });
    // console.log("Encoded:", encoded);
    // console.log("Raw key (base64):", derived);
  }
  const dispatch = createEventDispatcher();
</script>

<div class="flex flex-row h-8/10">
  <div class="flex-1 flex flex-col justify-center p-30">
    <h1 class=" text-9xl mx-3 font-bold title w-auto">Zero</h1>
    <h1 class="text-9xl mx-3 font-bold title w-auto">Point</h1>
    <h1 class="text-9xl mx-3 font-bold title w-auto">Four</h1>
  </div>
  <div
    class="flex-1 flex flex-col align-middle mx-5 h-full p-20 justify-center-safe liquid-container"
  >
    <h1 class="text-7xl my-10 title font-bold py-3">Log In</h1>
    <h2 class="title text-lg font-medium">Username</h2>
    <input
      placeholder="Enter Username"
      bind:value={username}
      class=" bg-amber-50 rounded-lg justify-self-center min-w-9/10 max-w-full py-3 my-3 p-2"
    />
    <h2 class="title text-lg font-medium">Master password</h2>
    <input
      type="password"
      placeholder="Enter master password"
      bind:value={password}
      class=" bg-amber-50 rounded-lg justify-self-center min-w-9/10 max-w-full py-3 my-3 p-2"
    />
    <button
      class=" bg-gradient-to-r from-white via-cyan-100 to-purple-200 min-w-9/10 max-w-full py-3 my-3 rounded-lg hover:rotate-2"
      on:click={onEnter}
    >
      Login
    </button>
    <p class="mt-4 text-center">
      Don't have an account?
      <a
        href="#"
        on:click={(e) => {
          e.preventDefault();
          dispatch("changeView", "register");
        }}
      >
        Register
      </a>
    </p>
  </div>
</div>
