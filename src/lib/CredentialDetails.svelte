<script lang="ts">
  import type { Credential } from "./types";
  export let item: Credential | null = null;
  export let onSave: (updated: Credential) => void;

  let showPassword: boolean = false;
  let isEditing: boolean = false;
  let editUsername: string = "";
  let editPassword: string = "";

  $: if (item) {
    editUsername = item.username;
    editPassword = item.password;
    showPassword = false;
    isEditing = false;
  }

  function togglePassword() {
    showPassword = !showPassword;
  }

  function startEdit() {
    if (!item) return;
    isEditing = true;
    editUsername = item.username;
    editPassword = item.password;
  }

  function cancelEdit() {
    isEditing = false;
    if (item) {
      editUsername = item.username;
      editPassword = item.password;
    }
  }

  function saveEdit() {
    if (!item) return;
    const updated: Credential = {
      website: item.website,
      username: editUsername,
      password: editPassword,
    };
    onSave(updated);
    isEditing = false;
  }
</script>

{#if item}
  <div class="p-5">
    <h2 class="text-5xl font-bold mb-5 title">Details</h2>
    {#if isEditing}
      <div class="flex flex-col gap-3 max-w-md">
        <label class="flex flex-col">
          <span class="opacity-70 mb-1">Username</span>
          <input
            class="bg-amber-50 rounded-lg p-2 text-black"
            bind:value={editUsername}
          />
        </label>
        <label class="flex flex-col">
          <span class="opacity-70 mb-1">Password</span>
          <input
            class="bg-amber-50 rounded-lg p-2 text-black"
            type="text"
            bind:value={editPassword}
          />
        </label>
        <div class="flex gap-2 mt-2">
          <button
            type="button"
            class="px-4 py-2 rounded-lg bg-white/10 hover:bg-white/20 focus:outline-none focus:ring-2 focus:ring-blue-300"
            on:click={saveEdit}>Save</button
          >
          <button
            type="button"
            class="px-4 py-2 rounded-lg bg-white/5 hover:bg-white/10 focus:outline-none focus:ring-2 focus:ring-slate-300"
            on:click={cancelEdit}>Cancel</button
          >
        </div>
      </div>
    {:else}
      <div class="mb-2 w-full">
        <h1
          class="text-lg bg-gradient-to-r from-white via-10% via-blue-200 to-purple-500 bg-clip-text font-bold mx-2 text-transparent"
        >
          Username
        </h1>
        <p class="ml-2 p-2 liquid-item rounded-lg my-3 text-white/80">
          {item.username}
        </p>
      </div>
      <div class="flex items-center gap-3 w-full">
        <div class="w-full">
          <h1
            class="text-lg bg-gradient-to-r from-white via-10% via-blue-200 to-purple-500 bg-clip-text font-bold mx-2 text-transparent"
          >
            Password
          </h1>
          <p class="ml-2 p-2 liquid-item rounded-lg my-3 text-white/80">
            {showPassword ? item.password : "••••••••"}
          </p>
        </div>
      </div>
      <div class="mt-4">
        <button
          type="button"
          class="px-4 py-2 rounded-lg bg-white/10 hover:bg-white/20 focus:outline-none focus:ring-2 focus:ring-blue-300 text-white"
          on:click={startEdit}>Edit</button
        >
        <button
          type="button"
          class="px-3 py-1 rounded bg-white/10 hover:bg-white/20 focus:outline-none focus:ring-2 focus:ring-blue-300 text-white"
          on:click={togglePassword}>{showPassword ? "Hide" : "Show"}</button
        >
      </div>
    {/if}
  </div>
{/if}
