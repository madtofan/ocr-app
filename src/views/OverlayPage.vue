<script setup lang="ts">
import { Button } from "@/components/ui/button";
import { Spinner } from "@/components/ui/spinner";
import {
    HoverCard,
    HoverCardContent,
    HoverCardTrigger,
} from "@/components/ui/hover-card";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { onMounted, onUnmounted, ref } from "vue";

let unlistenResetOcr: UnlistenFn | undefined;
let unlistenRunOcr: UnlistenFn | undefined;

interface OcrBox {
    text: string;
    x: number;
    y: number;
    width: number;
    height: number;
}

const ocrBoxes = ref<OcrBox[] | null>(null);

onMounted(async () => {
    unlistenResetOcr = await listen("reset-ocr", () => {
        ocrBoxes.value = null;
    });
    unlistenRunOcr = await listen<OcrBox[]>("run-ocr", (event) => {
        ocrBoxes.value = event.payload;
    });
});

onUnmounted(async () => {
    unlistenResetOcr?.();
    unlistenRunOcr?.();
});
</script>

<template>
    <div
        class="w-screen h-screen bg-black/0 flex"
        :class="ocrBoxes === null ? 'items-center justify-center' : ''"
    >
        <Button v-if="ocrBoxes === null" disabled class="bg-red-500 text-black">
            <Spinner />
            Loading Translation
        </Button>
        <div v-else>
            <template v-for="ocrBox in ocrBoxes">
                <HoverCard>
                    <HoverCardTrigger as-child>
                        <div
                            class="absolute border border-red-600 rounded-none"
                            :style="{
                                top: ocrBox.y + 'px',
                                left: ocrBox.x + 'px',
                                height: ocrBox.height + 'px',
                                width: ocrBox.width + 'px',
                            }"
                        />
                    </HoverCardTrigger>
                    <HoverCardContent class="w-80">
                        <div class="flex justify-between space-x-4">
                            <div class="space-y-1">
                                <h4 class="text-sm font-semibold">
                                    {{ ocrBox.text }}
                                </h4>
                                <p class="text-sm">
                                    This is the translation for the word
                                </p>
                            </div>
                        </div>
                    </HoverCardContent>
                </HoverCard>
            </template>
        </div>
    </div>
</template>
