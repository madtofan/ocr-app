<script setup lang="ts">
import { Card, CardContent, CardFooter } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { RefreshCcw, Image as ImageIcon } from "lucide-vue-next";
import { onMounted, onUnmounted } from "vue";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { useQueryClient } from "@tanstack/vue-query";
import { useScreenshots } from "@/composables/useScreenshots";

const queryClient = useQueryClient();

// Use our custom hook
const { data: screenshots, isLoading, isError, refresh } = useScreenshots();

let unlisten: UnlistenFn | undefined;
// Inside your function:
onMounted(async () => {
    unlisten = await listen("screenshot-taken", () => {
        // When Rust says "I took a picture", we tell TanStack Query to refetch
        queryClient.invalidateQueries({ queryKey: ["screenshots"] });
    });
});

onUnmounted(async () => {
    unlisten?.();
});

console.log({ screenshots });
</script>

<template>
    <div class="p-6 space-y-6">
        <div class="flex items-center justify-between">
            <h2 class="text-2xl font-bold tracking-tight">Gallery</h2>
            <Button variant="outline" size="sm" @click="refresh">
                <RefreshCcw
                    class="w-4 h-4 mr-2"
                    :class="{ 'animate-spin': isLoading }"
                />
                Refresh
            </Button>
        </div>

        <div v-if="isLoading" class="flex items-center justify-center h-40">
            <span class="text-muted-foreground"
                >Loading specific captures...</span
            >
        </div>

        <div v-else-if="isError" class="text-red-500">
            Failed to load screenshots. Check permissions.
        </div>

        <div
            v-else-if="!screenshots?.length"
            class="flex flex-col items-center justify-center h-64 border-2 border-dashed rounded-lg"
        >
            <ImageIcon class="w-10 h-10 text-muted-foreground mb-2" />
            <p class="text-muted-foreground">No screenshots taken yet.</p>
            <p class="text-xs text-muted-foreground mt-1">
                Press Ctrl+Shift+S to take one.
            </p>
        </div>

        <div
            v-else
            class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4"
        >
            <Card
                v-for="img in screenshots"
                :key="img.name"
                class="overflow-hidden group hover:shadow-lg transition-all"
            >
                <CardContent class="p-0 aspect-video relative">
                    <img
                        :src="img.url"
                        :alt="img.name"
                        class="object-cover w-full h-full"
                        loading="lazy"
                    />
                    <div
                        class="absolute inset-0 bg-black/40 opacity-0 group-hover:opacity-100 transition-opacity flex items-center justify-center"
                    >
                        <Button variant="secondary" size="sm">View</Button>
                    </div>
                </CardContent>
                <CardFooter class="p-2">
                    <p class="text-xs text-muted-foreground truncate w-full">
                        {{ img.name }}
                    </p>
                </CardFooter>
            </Card>
        </div>
    </div>
</template>
