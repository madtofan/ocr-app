<script setup lang="ts">
import {
    Sidebar,
    SidebarContent,
    SidebarFooter,
    SidebarGroup,
    SidebarGroupLabel,
    SidebarGroupContent,
    SidebarHeader,
    SidebarMenu,
    SidebarMenuButton,
    SidebarMenuItem,
} from "@/components/ui/sidebar";
import {
    BarChart3,
    BookOpen,
    GalleryVerticalEnd,
    GraduationCap,
    Home,
    Settings,
} from "lucide-vue-next";

const items = [
    {
        title: "Home",
        url: "/",
        icon: Home,
    },
    {
        title: "Word List",
        url: "/words",
        icon: BookOpen,
    },
    {
        title: "Study Mode",
        url: "/study",
        icon: GraduationCap,
    },
    {
        title: "Statistics",
        url: "/stats",
        icon: BarChart3,
    },
    {
        title: "Settings",
        url: "/settings",
        icon: Settings,
    },
];
</script>

<template>
    <Sidebar>
        <SidebarHeader class="pt-4">
            <SidebarMenu>
                <SidebarMenuItem>
                    <SidebarMenuButton size="lg">
                        <div
                            class="flex aspect-square size-8 items-center justify-center rounded-lg bg-sidebar-primary text-sidebar-primary-foreground"
                        >
                            <GalleryVerticalEnd class="size-4" />
                        </div>
                        <div
                            class="grid flex-1 text-left text-sm leading-tight"
                        >
                            <span class="truncate font-semibold"
                                >LangCapture</span
                            >
                            <span class="text-xs">Translation Capture</span>
                        </div>
                    </SidebarMenuButton>
                </SidebarMenuItem>
            </SidebarMenu>
        </SidebarHeader>
        <SidebarContent>
            <SidebarGroup>
                <SidebarGroupLabel>Pages</SidebarGroupLabel>
                <SidebarGroupContent>
                    <SidebarMenu>
                        <SidebarMenuItem
                            v-for="item in items"
                            :key="item.title"
                        >
                            <SidebarMenuButton
                                as-child
                                :is-active="item.url === $route.path"
                            >
                                <a :href="item.url">
                                    <component :is="item.icon" />
                                    <span>{{ item.title }}</span>
                                </a>
                            </SidebarMenuButton>
                        </SidebarMenuItem>
                    </SidebarMenu>
                </SidebarGroupContent>
            </SidebarGroup>
            <SidebarGroup />
        </SidebarContent>
        <SidebarFooter>
            <div class="rounded-lg bg-muted p-4">
                <p class="text-sm font-medium text-foreground">Quick Capture</p>
                <p class="mt-1 text-xs text-muted-foreground">
                    Press
                    <kbd
                        class="rounded bg-background px-1.5 py-0.5 text-xs font-semibold"
                        >Ctrl+Shift+S</kbd
                    >
                    to capture
                </p>
            </div>
        </SidebarFooter>
    </Sidebar>
</template>
