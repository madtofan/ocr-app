<script setup lang="ts">
import TodoManager from "@/components/TodoManager.vue";
import { pagePropertiesKey } from "@/lib/keys";
import { inject } from "vue";

const { updateHeader } = inject(pagePropertiesKey, {
    header: "",
    updateHeader: (_: string) => {},
});

updateHeader("Todo Page");
</script>

<template>
    <TodoManager />
</template>
