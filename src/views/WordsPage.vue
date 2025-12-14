<script setup lang="ts">
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import {
    Dialog,
    DialogContent,
    DialogHeader,
    DialogTitle,
    DialogTrigger,
} from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Word, WordGroup } from "@/lib/types";
import { FolderPlus, Plus, Search, Star, Trash2 } from "lucide-vue-next";
import { ref } from "vue";

const groups = ref<WordGroup[]>([]);

interface WordSearchParams {
    searchQuery: string;
    filterFavourites: boolean;
}

const searchParams = ref<WordSearchParams>({
    searchQuery: "",
    filterFavourites: false,
});
const updateSearchParams = (newSearchParams: Partial<WordSearchParams>) => {
    searchParams.value = { ...searchParams.value, ...newSearchParams };
};

interface Group {
    name: string;
    color: string;
}

const newGroupFormValues = ref<Group>({
    name: "",
    color: "",
});
const updateNewGroupFormValues = (newValues: Partial<Group>) => {
    newGroupFormValues.value = { ...newGroupFormValues.value, ...newValues };
};

// TODO - change this into form
const handleSubmitGroup = () => {
    updateNewGroupFormValues({ name: "", color: "" });
};

const ALL_GROUP = "all";
const filterGroup = ref(ALL_GROUP);
const updateFilterGroup = (newFilterGroup: string) => {
    filterGroup.value = newFilterGroup;
};

const filteredWords = ref<Word[]>([]);

const handleToggleFavouriteWord = (wordId: string) => {};

const getWordGroups = (word: Word): WordGroup[] => {
    return word.groupIds.reduce<WordGroup[]>((accumulatedGroup, groupId) => {
        const foundGroup = groups.value.find((group) => group.id === groupId);
        if (foundGroup) {
            return [...accumulatedGroup, foundGroup];
        }
        return accumulatedGroup;
    }, []);
};

const handleAddToGroup = (wordId: string, groupId: string) => {};

const handleDeleteWord = (wordId: string) => {};
</script>

<template>
    <div class="p-8">
        <div class="mb-8">
            <h1 class="text-3xl font-bold text-foreground">Word List</h1>
            <p class="mt-2 text-muted-foreground">
                Manage your vocabulary and organize words into groups
            </p>
        </div>

        <!-- {/* Search and Filters */} -->
        <div class="mb-6 flex flex-wrap gap-4">
            <div class="relative flex-1">
                <Search
                    class="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground"
                />
                <Input
                    placeholder="Search words..."
                    :value="searchParams.searchQuery"
                    :onchange="
                        (searchQuery: string) =>
                            updateSearchParams({ searchQuery })
                    "
                    class="pl-9"
                />
            </div>

            <Button
                :variant="searchParams.filterFavourites ? 'default' : 'outline'"
                :onchange="
                    () =>
                        updateSearchParams({
                            filterFavourites: !searchParams.filterFavourites,
                        })
                "
                class="gap-2"
            >
                <Star class="h-4 w-4" />
                Favorites
            </Button>

            <Dialog>
                <DialogTrigger asChild>
                    <Button variant="outline" class="gap-2 bg-transparent">
                        <FolderPlus class="h-4 w-4" />
                        New Group
                    </Button>
                </DialogTrigger>
                <DialogContent>
                    <DialogHeader>
                        <DialogTitle>Create New Group</DialogTitle>
                    </DialogHeader>
                    <div class="space-y-4 pt-4">
                        <div>
                            <Label for="groupName">Group Name</Label>
                            <Input
                                id="groupName"
                                :value="newGroupFormValues.name"
                                :onchange="
                                    (name: string) =>
                                        updateNewGroupFormValues({ name })
                                "
                                placeholder="e.g., Business Terms"
                                class="mt-2"
                            />
                        </div>
                        <div>
                            <Label for="groupColor">Color</Label>
                            <Input
                                id="groupColor"
                                type="color"
                                :value="newGroupFormValues.color"
                                :onchange="
                                    (color: string) =>
                                        updateNewGroupFormValues({ color })
                                "
                                class="mt-2 h-10"
                            />
                        </div>
                        <Button :onclick="handleSubmitGroup" class="w-full">
                            Create Group
                        </Button>
                    </div>
                </DialogContent>
            </Dialog>
        </div>

        <!-- {/* Groups Filter */} -->
        <div v-if="groups.length > 0" class="mb-6 flex flex-wrap gap-2">
            <Button
                :variant="filterGroup === ALL_GROUP ? 'default' : 'outline'"
                size="sm"
                :onclick="() => updateFilterGroup(ALL_GROUP)"
            >
                All Words
            </Button>
            <Button
                v-for="group in groups"
                :key="group.id"
                :variant="filterGroup === group.id ? 'default' : 'outline'"
                size="sm"
                :onclick="() => updateFilterGroup(group.id)"
                :class="`border-[#${group.color}] ${filterGroup === group.id ? `bg-[#${group.color}]` : ''}`"
            >
                {{ group.name }}
            </Button>
        </div>

        <!-- {/* Words Grid */} -->
        <Card v-if="filteredWords.length === 0" class="p-12 text-center">
            <p class="text-sm font-medium text-foreground">No words found</p>
            <p class="mt-1 text-sm text-muted-foreground">
                {{
                    searchParams.searchQuery
                        ? "Try a different search term"
                        : "Start capturing words to build your vocabulary"
                }}
            </p>
        </Card>
        <div v-else class="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
            <Card
                v-for="word in filteredWords"
                :key="word.id"
                class="overflow-hidden"
            >
                <img
                    :src="word.screenshot"
                    :alt="word.word"
                    class="h-32 w-full object-cover"
                />
                <div class="p-4">
                    <div class="mb-2 flex items-start justify-between">
                        <h3 class="text-lg font-semibold text-foreground">
                            {{ word.word }}
                        </h3>
                        <Button
                            variant="ghost"
                            size="sm"
                            :onclick="handleToggleFavouriteWord"
                        >
                            <Star
                                :class="`h-4 w-4 ${word.isFavorite ? 'fill-yellow-400 text-yellow-400' : ''}`"
                            />
                        </Button>
                    </div>
                    <p class="mb-3 text-sm text-muted-foreground line-clamp-2">
                        {{ word.meaning }}
                    </p>
                    <div
                        v-if="word.groupIds.length > 0"
                        class="mb-3 flex flex-wrap gap-1"
                    >
                        <Badge
                            v-for="group in getWordGroups(word)"
                            :key="group.id"
                            :class="`text-xs text-white bg-[#${group.color}`"
                        >
                            {{ group.name }}
                        </Badge>
                        )
                    </div>
                    <div class="flex gap-2">
                        <Dialog>
                            <DialogTrigger asChild>
                                <Button
                                    variant="outline"
                                    size="sm"
                                    class="flex-1 bg-transparent"
                                >
                                    <Plus class="mr-1 h-3 w-3" />
                                    Add to Group
                                </Button>
                            </DialogTrigger>
                            <DialogContent>
                                <DialogHeader>
                                    <DialogTitle>Add to Group</DialogTitle>
                                </DialogHeader>
                                <div class="space-y-2 pt-4">
                                    <Button
                                        v-for="group in groups"
                                        :key="group.id"
                                        variant="outline"
                                        class="w-full justify-start bg-transparent"
                                        :onclick="
                                            () =>
                                                handleAddToGroup(
                                                    word.id,
                                                    group.id,
                                                )
                                        "
                                        :disabled="
                                            word.groupIds.includes(group.id)
                                        "
                                    >
                                        <div
                                            class="mr-2 h-4 w-4 rounded-full"
                                            :class="`bg-[#${group.color}]`"
                                        />
                                        {{ group.name }}
                                    </Button>
                                </div>
                            </DialogContent>
                        </Dialog>

                        <Button
                            variant="ghost"
                            size="sm"
                            :onclick="() => handleDeleteWord(word.id)"
                        >
                            <Trash2 class="h-4 w-4 text-destructive" />
                        </Button>
                    </div>
                </div>
            </Card>
        </div>
    </div>
</template>
