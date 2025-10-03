<script lang="ts">
  import type { Credential } from "./types";
  export let open: boolean = false;
  export let onClose: () => void;
  export let onCreate: (item: Credential) => void;

  let website: string = "";
  let username: string = "";
  let password: string = "";

  function randomSecurePassword(length: number = 32): string {
    const alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    const symbols = "!@#$%^&*()_+{}[]|:;<>,.?/~`-=";
    const chars = alphabet + symbols;
    const array = new Uint32Array(length);
    crypto.getRandomValues(array);
    let out = "";
    for (let i = 0; i < length; i++) {
      out += chars[array[i] % chars.length];
    }
    return out;
  }

  function handleGenerate() {
    password = randomSecurePassword(32);
  }

  function handleSubmit() {
    const trimmedWebsite = website.trim();
    if (!trimmedWebsite) return;
    onCreate({ website: trimmedWebsite, username: username.trim(), password: password.trim() });
    website = "";
    username = "";
    password = "";
    onClose();
  }
</script>

{#if open}
  <div class="fixed inset-0 bg-black/60 flex items-center justify-center z-50">
    <div class="bg-white/90 rounded-xl p-6 min-w-[320px] max-w-[90vw] text-black">
      <h3 class="text-xl font-semibold mb-4">New Credential</h3>
      <div class="flex flex-col gap-3">
        <label class="flex flex-col">
          <span class="text-sm opacity-80 mb-1">Website</span>
          <input class="rounded-lg p-2 border" bind:value={website} placeholder="example.com" />
        </label>
        <label class="flex flex-col">
          <span class="text-sm opacity-80 mb-1">Username</span>
          <input class="rounded-lg p-2 border" bind:value={username} placeholder="user@example.com" />
        </label>
        <label class="flex flex-col">
          <span class="text-sm opacity-80 mb-1">Password</span>
          <div class="flex gap-2">
            <input class="rounded-lg p-2 border flex-1" bind:value={password} placeholder="********" />
            <button type="button" class="px-3 py-2 rounded bg-slate-800 text-white hover:bg-slate-700" on:click={handleGenerate}>Generate</button>
          </div>
        </label>
      </div>
      <div class="flex justify-end gap-2 mt-5">
        <button type="button" class="px-4 py-2 rounded bg-slate-200 hover:bg-slate-300" on:click={onClose}>Cancel</button>
        <button type="button" class="px-4 py-2 rounded bg-blue-600 text-white hover:bg-blue-500" on:click={handleSubmit}>Create</button>
      </div>
    </div>
  </div>
{/if}


