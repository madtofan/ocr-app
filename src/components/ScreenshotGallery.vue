<script setup lang="ts">
import { onMounted, onUnmounted, ref } from "vue";
import { listen, UnlistenFn, Event } from "@tauri-apps/api/event";

const base64image = ref("");

let unlisten: UnlistenFn | undefined;
onMounted(async () => {
    unlisten = await listen<string>(
        "screenshot-taken",
        (image: Event<string>) => {
            // When Rust says "I took a picture", we update the screenshot
            base64image.value = image.payload;
        },
    );
});

onUnmounted(async () => {
    unlisten?.();
});
</script>

<template>
    <div class="p-6 space-y-6">
        <div class="flex items-center justify-between">
            <h2 class="text-2xl font-bold tracking-tight">Gallery</h2>
        </div>

        <div v-if="base64image">
            <img
                :src="`data:image/png;base64,${base64image}`"
                alt="Base64 Image"
            />
        </div>
    </div>
</template>
