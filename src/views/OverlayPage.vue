<script setup lang="ts">
import { CalendarDaysIcon } from "lucide-vue-next";
import { Button } from "@/components/ui/button";
import {
    HoverCard,
    HoverCardContent,
    HoverCardTrigger,
} from "@/components/ui/hover-card";
</script>

<template>
    <div class="w-screen h-screen bg-black/0 flex items-center justify-center">
        <HoverCard>
            <HoverCardTrigger as-child>
                <Button variant="link"> @nuxt </Button>
            </HoverCardTrigger>
            <HoverCardContent class="w-80">
                <div class="flex justify-between space-x-4">
                    <div class="space-y-1">
                        <h4 class="text-sm font-semibold">@nuxt</h4>
                        <p class="text-sm">
                            The Vue Framework – The Progressive Web Framework.
                        </p>
                        <div class="flex items-center pt-2">
                            <CalendarDaysIcon class="mr-2 h-4 w-4 opacity-70" />
                            <span class="text-xs text-muted-foreground">
                                Joined December 2021
                            </span>
                        </div>
                    </div>
                </div>
            </HoverCardContent>
        </HoverCard>
    </div>
</template>
