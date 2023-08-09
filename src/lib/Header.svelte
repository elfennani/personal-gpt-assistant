<script lang="ts">
    import CloseIcon from "~icons/mdi/close";
    import { appWindow } from "@tauri-apps/api/window";
    import { link, useLocation } from "svelte-routing";
    import { onDestroy } from "svelte";
    import LeftIcon from "~icons/mdi/chevron-left";

    let location: string;

    let unsubscribe = useLocation().subscribe((l) => (location = l.pathname));

    $: {
        console.log(location);
    }

    onDestroy(unsubscribe);
</script>

<div class="px-5 h-16 flex items-center justify-between">
    {#if location == "/"}
        <h1 class="leading-loose text-2xl">Chat Assistant</h1>
    {:else}
        <a href="/" class="flex items-center py-2" use:link><LeftIcon /> Back</a
        >
    {/if}
    <button
        class="p-1 hover:bg-zinc-800 rounded-full text-zinc-400"
        on:click={appWindow.close}
    >
        <CloseIcon />
    </button>
</div>
