<script lang="ts">
    import { onMount } from "svelte";
    import ISend from "~icons/mdi/send";

    let textArea: HTMLTextAreaElement;
    let message: string = "";

    function handleSize() {
        textArea.style.height = "1.5rem";
        textArea.style.height = `${textArea.scrollHeight}px`;
    }

    function focusInput(ev: KeyboardEvent) {
        if (ev.key == "/") textArea.focus();
    }

    onMount(handleSize);

    function handleKeyPress(
        event: KeyboardEvent & {
            currentTarget: EventTarget & HTMLTextAreaElement;
        }
    ) {
        if (event.key == "Enter") {
            if (event.shiftKey) return;
            event.preventDefault();
            message = "";
        }
    }
</script>

<svelte:window on:resize={handleSize} on:keyup={focusInput} />
<form on:submit|preventDefault class="p-5">
    <!-- svelte-ignore a11y-autofocus -->
    <div class="bg-zinc-700 rounded-md flex border border-zinc-500">
        <textarea
            name="question"
            bind:this={textArea}
            bind:value={message}
            placeholder="Ask me anything"
            on:input={handleSize}
            on:keypress={handleKeyPress}
            class="resize-none text-white flex-1 bg-transparent p-4 outline-none placeholder:text-zinc-400"
            autofocus
        />
        <button
            class="bg-teal-500 disabled:bg-zinc-500 text-white m-2 w-10 h-10 flex items-center justify-center rounded"
            disabled={message.trim() == ""}
        >
            <ISend />
        </button>
    </div>
</form>
