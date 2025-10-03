<script lang="ts">
  export let onAddNew: () => void;
</script>

<div class="flex flex-row items-center justify-between">
  <h1
    class="text-4xl bg-gradient-to-r from-white via-10% via-blue-200 to-purple-500 bg-clip-text font-bold mx-2 text-transparent"
  >
    Vault
  </h1>
  <button
    type="button"
    class="mx-2 px-4 py-2 rounded-lg bg-white/10 hover:bg-white/20 text-white/90 focus:outline-none focus:ring-2 focus:ring-blue-300"
    on:click={onAddNew}
  >
    New
  </button>
</div>
