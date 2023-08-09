import { writable } from "svelte/store";

export interface ChatMessage {
    content: string;
    type: "function" | "assistant" | "user";
}

export const messages = writable<ChatMessage[]>([]);
