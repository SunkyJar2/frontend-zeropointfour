<script lang="ts">
  import "../app.css";
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import type { Credential } from "./types";
  import VaultHeader from "./VaultHeader.svelte";
  import CredentialList from "./CredentialList.svelte";
  import CredentialDetails from "./CredentialDetails.svelte";
  import NewCredentialModal from "./NewCredentialModal.svelte";

  let credentials: Credential[] = [];
  let loadError: string = "";
  let selected: Credential | null = null;
  // state driven by details component via selected

  onMount(async () => {
    try {
      const raw = await invoke<string>("load_credentials");
      credentials = JSON.parse(raw);
    } catch (error) {
      loadError = String(error);
    }
  });

  function handleSelect(item: Credential) {
    selected = item;
  }

  function saveEdit(updated: Credential) {
    if (!selected) return;
    const index = credentials.findIndex(
      (c) =>
        c.website === selected?.website && c.username === selected?.username
    );
    if (index !== -1) {
      credentials[index] = { ...credentials[index], ...updated };
      credentials = [...credentials];
      selected = credentials[index];
      persist();
    }
  }

  function addNewCredential() {
    openNewModal = true;
  }

  async function persist() {
    try {
      await invoke("save_credentials", { json: JSON.stringify(credentials) });
    } catch (error) {
      console.error("Failed to save credentials:", error);
    }
  }

  let openNewModal: boolean = false;
  function handleCreate(item: Credential) {
    credentials = [item, ...credentials];
    handleSelect(item);
    persist();
  }
</script>

<div
  class="bg-black/50 w-full h-[93vh] flex flex-row overflow-hidden p-5 py-10"
>
  <div class="liquid-container flex-1 p-5">
    <VaultHeader onAddNew={addNewCredential} />
    {#if loadError}
      <p class="text-red-400 mx-2 mt-2">{loadError}</p>
    {/if}
    <CredentialList items={credentials} onSelect={handleSelect} />
  </div>
  <div class=" flex-1">
    <CredentialDetails item={selected} onSave={saveEdit} />
  </div>
</div>

<NewCredentialModal
  open={openNewModal}
  onClose={() => (openNewModal = false)}
  onCreate={handleCreate}
/>
