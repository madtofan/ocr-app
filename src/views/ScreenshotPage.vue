<script setup lang="ts">
import ScreenshotGallery from "@/components/ScreenshotGallery.vue";
import { pagePropertiesKey } from "@/lib/keys";
import { inject } from "vue";

const { updateHeader } = inject(pagePropertiesKey, {
    header: "",
    updateHeader: (_: string) => {},
});

updateHeader("Screenshot Gallery");
</script>

<template>
    <ScreenshotGallery />
</template>
