<script setup lang="ts">
import {
    SidebarInset,
    SidebarProvider,
    SidebarTrigger,
} from "@/components/ui/sidebar";
import AppSidebar from "@/components/AppSidebar.vue";
import { Toaster } from "@/components/ui/sonner";
import { onMounted, onUnmounted, provide, ref } from "vue";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { pagePropertiesKey } from "@/lib/keys";

let unlisten: UnlistenFn | undefined;

const header = ref("");

const updateHeader = (newHeader: string) => {
    header.value = newHeader;
};

provide(pagePropertiesKey, {
    header,
    updateHeader,
});

// Inside your function:
onMounted(async () => {
    unlisten = await listen("request-minimize", () => {});
});

onUnmounted(async () => {
    unlisten?.();
});
</script>

<template>
    <SidebarProvider>
        <AppSidebar />
        <SidebarInset>
            <header
                class="flex h-16 shrink-0 items-center gap-2 transition-[width,height] ease-linear group-has-data-[collapsible=icon]/sidebar-wrapper:h-12"
            >
                <header class="flex items-center gap-2 px-4">
                    <SidebarTrigger class="-ml-1" />
                    <h1>{{ header }}</h1>
                </header>
            </header>
            <div class="flex flex-1 flex-col gap-4 p-4 pt-0">
                <main
                    class="min-h-screen flex-1 rounded-xl bg-muted/50 md:min-h-min p-5"
                >
                    <router-view />
                </main>
            </div>
            <Toaster />
        </SidebarInset>
    </SidebarProvider>
</template>
