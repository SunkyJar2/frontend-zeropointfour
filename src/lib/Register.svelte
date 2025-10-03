<script lang="ts">
  import "../app.css";
  import { invoke } from "@tauri-apps/api/core";
  import { createEventDispatcher } from "svelte";
  let password = "";
  let username = "";
  let response = "";
  let errorMessage = "";

  async function onEnter() {
    let messages = [];
    if (password.length < 14) {
      messages.push("Password is too short (should be at least 14 characters)");
    }
    if (!/[A-Z]/.test(password)) {
      messages.push("Password should contain at least one uppercase letter");
    }
    if (!/[a-z]/.test(password)) {
      messages.push("Password should contain at least one lowercase letter");
    }
    if (!/[0-9]/.test(password)) {
      messages.push("Password should contain at least one number");
    }
    if (!/[^A-Za-z0-9]/.test(password)) {
      messages.push("Password should contain at least one special symbol");
    }
    if (messages.length) {
      errorMessage = messages.join(". ");
      return;
    }
    errorMessage = "";

    // Replace direct response assignment with additional error checking and JSON parsing if needed
    const res: any = await invoke("register", { username, password });
    const result = typeof res === "string" ? JSON.parse(res) : res;
    if (
      result &&
      result.success === false &&
      result.message === "User already exists"
    ) {
      errorMessage = result.message;
      return;
    }
    response = result;
    console.log("Response:", response);
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
    <h1 class="text-7xl my-10 title font-bold py-3">Register</h1>
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
      on:keydown={(e) => e.key === "Enter" && onEnter()}
      class=" bg-amber-50 rounded-lg justify-self-center min-w-9/10 max-w-full py-3 my-3 p-2"
    />
    {#if errorMessage}
      <p class="error">{errorMessage}</p>
    {/if}
    <button
      class=" bg-gradient-to-r from-white via-cyan-100 to-purple-200 min-w-9/10 max-w-full py-3 my-3 rounded-lg hover:rotate-2"
      on:click={onEnter}
    >
      Register
    </button>
    <p class="mt-4 text-center">
      Already have an account?
      <a
        href="#"
        on:click={(e) => {
          e.preventDefault();
          dispatch("changeView", "login");
        }}
      >
        Log In
      </a>
    </p>
  </div>
</div>
