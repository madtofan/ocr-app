<script setup lang="ts">
import { ref } from "vue";
import { useTodos } from "@/composables/useTodos";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Checkbox } from "@/components/ui/checkbox"; // Run: bunx shadcn-vue@latest add checkbox
import { Trash2, Plus } from "lucide-vue-next";

const { todos, isLoading, error, addTodo, toggleTodo, deleteTodo } = useTodos();
const newTodoInput = ref("");

const handleSubmit = () => {
    if (!newTodoInput.value.trim()) return;
    addTodo.mutate(newTodoInput.value);
    newTodoInput.value = "";
};
</script>

<template>
    <div class="max-w-md mx-auto mt-10 space-y-6">
        <div class="space-y-2">
            <h2 class="text-2xl font-bold">Project Tasks</h2>
            <p class="text-muted-foreground text-sm">
                Stored locally in SQLite
            </p>
        </div>

        <form @submit.prevent="handleSubmit" class="flex gap-2">
            <Input
                v-model="newTodoInput"
                placeholder="What needs to be done?"
                class="flex-1"
            />
            <Button type="submit" :disabled="addTodo.isPending.value">
                <Plus class="w-4 h-4 mr-2" />
                Add
            </Button>
        </form>

        <div v-if="isLoading" class="text-center py-4 text-muted-foreground">
            Loading database...
        </div>

        <div v-else-if="error" class="text-center py-4 text-muted-foreground">
            Error: {{ error }}
        </div>

        <div v-else class="space-y-2">
            <div
                v-for="todo in todos"
                :key="todo.id"
                class="flex items-center justify-between p-3 border rounded-lg bg-card text-card-foreground shadow-sm animate-in fade-in slide-in-from-bottom-2"
            >
                <div class="flex items-center gap-3">
                    <Checkbox
                        :checked="todo.status === 'done'"
                        @update:checked="() => toggleTodo.mutate(todo)"
                    />
                    <span
                        :class="{
                            'line-through text-muted-foreground':
                                todo.status === 'done',
                        }"
                    >
                        {{ todo.title }}
                    </span>
                </div>

                <Button
                    variant="ghost"
                    size="icon"
                    class="text-destructive hover:text-destructive/90 hover:bg-destructive/10"
                    @click="deleteTodo.mutate(todo.id)"
                >
                    <Trash2 class="w-4 h-4" />
                </Button>
            </div>

            <div
                v-if="todos?.length === 0"
                class="text-center py-8 text-muted-foreground border-dashed border-2 rounded-lg"
            >
                No tasks yet. Start by adding one!
            </div>
        </div>
    </div>
</template>
