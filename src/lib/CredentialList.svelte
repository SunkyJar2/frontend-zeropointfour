<script lang="ts">
  import type { Credential } from "./types";
  export let items: Credential[] = [];
  export let onSelect: (item: Credential) => void;
  function handleSelect(item: Credential) {
    onSelect(item);
  }
</script>

{#if items.length > 0}
  <ul class="mx-2 mt-4 space-y-2">
    {#each items as item, index}
      <li class="rounded-lg bg-white/5 text-white/90">
        <button
          type="button"
          class="w-full text-left p-3 flex flex-row justify-between items-center hover:bg-white/10 focus:outline-none focus:ring-2 focus:ring-blue-300 rounded-lg"
          on:click={() => handleSelect(item)}
          on:keydown={(e) => {
            if (e.key === "Enter" || e.key === " ") {
              e.preventDefault();
              handleSelect(item);
            }
          }}
        >
          <div class="flex flex-col">
            <span class="font-medium">{item.website}</span>
            <span class="opacity-90">{item.username}</span>
          </div>
        </button>
      </li>
    {/each}
  </ul>
{:else}
  <p class="mx-2 mt-4 opacity-70">No credentials to display.</p>
{/if}
