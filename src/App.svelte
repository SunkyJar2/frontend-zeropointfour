<script lang="ts">
  import "./app.css";
  import { invoke } from "@tauri-apps/api/core";
</script>

<main class=" h-dvh bg-[url('/bg.png')] bg-cover">
  <div class="bg-transparent py-5"></div>

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
      <h2 class="title text-lg font-medium">Master password</h2>
      <input
        type="password"
        class=" bg-amber-50 rounded-lg justify-self-center min-w-9/10 max-w-full py-3 my-3"
      />
      <button
        class=" bg-gradient-to-r from-white via-cyan-100 to-purple-200 min-w-9/10 max-w-full py-3 my-3 rounded-lg"
      >
        Log In
      </button>
    </div>
  </div>
</main>
