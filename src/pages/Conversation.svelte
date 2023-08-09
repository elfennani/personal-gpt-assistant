<script lang="ts">
    import { Body } from "svelte-body";
    import ConvContent from "../lib/ConvContent.svelte";
    import Input from "../lib/Input.svelte";
    import { onMount } from "svelte";
    import { navigate } from "svelte-routing";
    import { invoke } from "@tauri-apps/api/tauri";
    import type AppChat from "../types/AppChat";

    export let id: string = null;

    onMount(async () => {
        if (!id) {
            try {
                const new_chat: AppChat = await invoke("new_chat");
                navigate(`/${new_chat.id}`, { replace: true });
            } catch (error) {
                console.error(error);
            }
        }
    });
</script>

<Body class="bg-zinc-800" />
<div class="grid grid-rows-[1fr_min-content] h-full text-zinc-400 flex-1">
    <ConvContent isLoading={!id} />
    <Input />
</div>
