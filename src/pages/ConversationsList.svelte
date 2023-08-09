<script lang="ts">
    import MessageIcon from "~icons/mdi/message-outline";
    import RightIcon from "~icons/mdi/chevron-right";
    import { link } from "svelte-routing";
    import { Body } from "svelte-body";
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/tauri";

    /*
        struct AppChat {
            id: String,
            title: String,
            messages: Option<ChatCompletion>,
        }
    */

    interface AppChat {
        id: string;
        title: string;
        messages: string[];
    }

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
                <li>
                    <a
                        class="p-4 block hover:bg-zinc-800"
                        href={`/${chat.id}`}
                        use:link
                    >
                        {chat.title}
                    </a>
                </li>
            {/each}
        {/if}
    </ul>
</div>
