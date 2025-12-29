<script setup lang="ts">
import { onMounted, onUnmounted, ref } from "vue";
import { listen, UnlistenFn } from "@tauri-apps/api/event";

const base64images1 = ref<string[]>([]);
const base64images2 = ref<string[]>([]);

let unlisten1: UnlistenFn | undefined;
let unlisten2: UnlistenFn | undefined;
onMounted(async () => {
    unlisten1 = await listen<string[]>("base64-images1", (event) => {
        base64images1.value = event.payload;
    });
    unlisten2 = await listen<string[]>("base64-images2", (event) => {
        base64images2.value = event.payload;
    });
});

onUnmounted(async () => {
    unlisten1?.();
    unlisten2?.();
});
</script>

<template>
    <div class="p-6 space-y-6">
        <div class="flex items-center justify-between">
            <h2 class="text-2xl font-bold tracking-tight">mod.rs</h2>
        </div>
        <div v-for="image in base64images1" :key="image">
            <img :src="`${image}`" alt="Base64 Image" />
        </div>

        <div class="flex items-center justify-between">
            <h2 class="text-2xl font-bold tracking-tight">pp_ocr.rs</h2>
        </div>
        <div v-for="image in base64images2" :key="image">
            <img :src="`${image}`" alt="Base64 Image" />
        </div>
    </div>
</template>
