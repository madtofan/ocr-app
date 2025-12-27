<script setup lang="ts">
import {
    SidebarInset,
    SidebarProvider,
    SidebarTrigger,
} from "@/components/ui/sidebar";
import AppSidebar from "@/components/AppSidebar.vue";
import { onMounted, onUnmounted, provide, ref } from "vue";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { pagePropertiesKey } from "@/lib/keys";
import {
    AlertDialog,
    AlertDialogContent,
    AlertDialogHeader,
    AlertDialogTitle,
    AlertDialogDescription,
    AlertDialogFooter,
    AlertDialogCancel,
    AlertDialogAction,
} from "./ui/alert-dialog";
import { invoke } from "@tauri-apps/api/core";
import { exit } from "@tauri-apps/plugin-process";
import { toast } from "vue-sonner";

const header = ref("");
const showCloseDialog = ref(false);

const updateHeader = (newHeader: string) => {
    header.value = newHeader;
};

const handleCancelClose = async () => {
    showCloseDialog.value = false;
    await exit(0);
};

const handleClose = () => {
    showCloseDialog.value = false;
    invoke("hide_app_window");
};

provide(pagePropertiesKey, {
    header,
    updateHeader,
});

let unlistenError: UnlistenFn | undefined;
let unlistenMinimize: UnlistenFn | undefined;

// Inside your function:
onMounted(async () => {
    unlistenMinimize = await listen("request-minimize", () => {
        showCloseDialog.value = true;
    });
    unlistenError = await listen<string>("error", (event) => {
        toast("⛔️ An error has occured!", {
            description: event.payload,
            action: {
                label: "Clear",
            },
        });
    });
});

onUnmounted(async () => {
    unlistenMinimize?.();
    unlistenError?.();
});
</script>

<template>
    <SidebarProvider>
        <AlertDialog :open="showCloseDialog">
            <AlertDialogContent>
                <AlertDialogHeader>
                    <AlertDialogTitle>Hide the window</AlertDialogTitle>
                    <AlertDialogDescription>
                        Should LangCapture continue to run in the background to
                        enable the usage of translate shortcut key?
                    </AlertDialogDescription>
                </AlertDialogHeader>
                <AlertDialogFooter>
                    <AlertDialogCancel :onclick="handleCancelClose"
                        >No, close LangCapture</AlertDialogCancel
                    >
                    <AlertDialogAction :onclick="handleClose"
                        >Yes, run in the background</AlertDialogAction
                    >
                </AlertDialogFooter>
            </AlertDialogContent>
        </AlertDialog>
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
        </SidebarInset>
    </SidebarProvider>
</template>
