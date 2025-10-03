<script lang="ts">
  import "./app.css";
  import { invoke } from "@tauri-apps/api/core";
  import Login from "./lib/Login.svelte";
  import Register from "./lib/Register.svelte";
  import Vault from "./lib/Vault.svelte";

  let view: "login" | "register" = "login";
</script>

<main class="h-screen overflow-hidden bg-[url('/bg.png')] bg-cover mx-auto">
  <div class="bg-transparent py-3">
    <h1
      class="text-4xl bg-gradient-to-r from-white via-10% via-blue-200 to-purple-500 bg-clip-text font-bold mx-2 text-transparent"
    >
      ZeroPointFour
    </h1>
  </div>
  <!-- {#if view === "login"}
    <Login on:changeView={(e) => (view = e.detail)} />
  {:else}
    <Register on:changeView={(e) => (view = e.detail)} />
  {/if} -->

  <Vault />
</main>
