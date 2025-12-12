<script setup lang="ts">
import { pagePropertiesKey } from "@/lib/keys";
import { inject } from "vue";

const { updateHeader } = inject(pagePropertiesKey, {
    header: "",
    updateHeader: (_: string) => {},
});

updateHeader("Settings Page");
</script>

<template>
    <p>this is the setting page</p>
</template>
