<script lang="ts">
    import MessageIcon from "~icons/mdi/message-outline";
    import RightIcon from "~icons/mdi/chevron-right";
    import DeleteIcon from "~icons/mdi/delete";
    import EditIcon from "~icons/mdi/pencil";
    import { link } from "svelte-routing";
    import { Body } from "svelte-body";
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/tauri";
    import type AppChat from "../types/AppChat";

    let chats: AppChat[] | null = null;

    onMount(async () => {
        try {
            const chats_loaded: AppChat[] = await invoke("get_chats");

            chats = chats_loaded;
        } catch (error) {
            console.error(`failed to get chats ${error}`);
        }
    });
</script>

<Body class="bg-zinc-900" />
<div>
    <a
        href="/new"
        class="p-5 flex items-center justify-between hover:bg-zinc-800 border-b border-zinc-700"
        use:link
    >
        <span class="flex items-center gap-4 font-semibold">
            <MessageIcon class="text-lg" /> New Chat
        </span>
        <RightIcon />
    </a>
    <ul class="">
        {#if chats}
            {#each chats as chat}
                <li class="flex hover:bg-zinc-800 items-center pr-4">
                    <a class="p-4 block flex-1" href={`/${chat.id}`} use:link>
                        {chat.title}
                    </a>
                    <div class="flex items-center">
                        <a
                            href="#edit"
                            class="text-lg text-zinc-700 p-2 hover:text-zinc-100"
                            title="Edit"
                        >
                            <EditIcon />
                        </a>
                        <a
                            href="#delete"
                            class="text-lg text-zinc-700 p-2 hover:text-zinc-100"
                            title="Delete"
                        >
                            <DeleteIcon />
                        </a>
                    </div>
                </li>
            {/each}
        {:else}
            <li>Loading...</li>
        {/if}
    </ul>
</div>
