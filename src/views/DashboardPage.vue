<script setup lang="ts">
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { pagePropertiesKey } from "@/lib/keys";
import { UserStats, Word } from "@/lib/types";
import {
    BookOpen,
    Camera,
    FolderOpen,
    Star,
    TrendingUp,
} from "lucide-vue-next";
import { inject, ref } from "vue";
import { RouterLink } from "vue-router";

const recentWords = ref<Word[]>([]);
const stats = ref<UserStats>({
    totalWords: 0,
    wordsThisWeek: 0,
    wordsThisMonth: 0,
    favoriteCount: 0,
    groupCount: 0,
    studySessions: 0,
});
const { updateHeader } = inject(pagePropertiesKey, {
    header: "",
    updateHeader: (_: string) => {},
});

const handleQuickCapture = () => {};

updateHeader("Home");
</script>

<template>
    <div class="p-8">
        <!-- {/* Header */} -->
        <div class="mb-8">
            <h1 class="text-3xl font-bold text-balance text-foreground">
                Welcome back!
            </h1>
            <p class="mt-2 text-muted-foreground">
                Track your language learning progress and capture new words
            </p>
        </div>

        <!-- {/* Quick Actions */} -->
        <div class="mb-8 flex gap-4">
            <Button size="lg" :onclick="handleQuickCapture" class="gap-2">
                <Camera class="h-5 w-5" />
                Quick Capture
            </Button>
            <Button
                size="lg"
                variant="outline"
                asChild
                class="gap-2 bg-transparent"
            >
                <RouterLink to="/words">
                    <BookOpen class="h-5 w-5" />
                    Browse Words
                </RouterLink>
            </Button>
            <Button
                size="lg"
                variant="outline"
                asChild
                class="gap-2 bg-transparent"
            >
                <RouterLink to="/study">
                    <TrendingUp class="h-5 w-5" />
                    Start Studying
                </RouterLink>
            </Button>
        </div>

        <!-- {/* Stats Grid */} -->
        <div class="mb-8 grid gap-6 md:grid-cols-2 lg:grid-cols-4">
            <Card class="p-6">
                <div class="flex items-center justify-between">
                    <div>
                        <p class="text-sm font-medium text-muted-foreground">
                            Total Words
                        </p>
                        <p class="mt-2 text-3xl font-bold text-foreground">
                            {{ stats.totalWords }}
                        </p>
                    </div>
                    <BookOpen class="h-8 w-8 text-muted-foreground" />
                </div>
            </Card>

            <Card class="p-6">
                <div class="flex items-center justify-between">
                    <div>
                        <p class="text-sm font-medium text-muted-foreground">
                            This Week
                        </p>
                        <p class="mt-2 text-3xl font-bold text-foreground">
                            {{ stats.wordsThisWeek }}
                        </p>
                    </div>
                    <TrendingUp class="h-8 w-8 text-muted-foreground" />
                </div>
            </Card>

            <Card class="p-6">
                <div class="flex items-center justify-between">
                    <div>
                        <p class="text-sm font-medium text-muted-foreground">
                            Favorites
                        </p>
                        <p class="mt-2 text-3xl font-bold text-foreground">
                            {{ stats.favoriteCount }}
                        </p>
                    </div>
                    <Star class="h-8 w-8 text-muted-foreground" />
                </div>
            </Card>

            <Card class="p-6">
                <div class="flex items-center justify-between">
                    <div>
                        <p class="text-sm font-medium text-muted-foreground">
                            Groups
                        </p>
                        <p class="mt-2 text-3xl font-bold text-foreground">
                            {{ stats.groupCount }}
                        </p>
                    </div>
                    <FolderOpen class="h-8 w-8 text-muted-foreground" />
                </div>
            </Card>
        </div>

        <!-- {/* Recent Words */} -->
        <Card class="p-6">
            <div class="mb-4 flex items-center justify-between">
                <h2 class="text-xl font-semibold text-foreground">
                    Recent Words
                </h2>
                <Button variant="ghost" size="sm" asChild>
                    <RouterLink to="/words">View all</RouterLink>
                </Button>
            </div>

            <div
                v-if="recentWords.length === 0"
                class="flex flex-col items-center justify-center py-12 text-center"
            >
                <BookOpen class="mb-4 h-12 w-12 text-muted-foreground" />
                <p class="text-sm font-medium text-foreground">No words yet</p>
                <p class="mt-1 text-sm text-muted-foreground">
                    Start capturing words to build your vocabulary
                </p>
                <Button class="mt-4" :onclick="handleQuickCapture">
                    <Camera class="mr-2 h-4 w-4" />
                    Capture Your First Word
                </Button>
            </div>
            <div v-else class="space-y-4">
                <div
                    v-for="word in recentWords"
                    :key="word.id"
                    class="flex items-start gap-4 rounded-lg border border-border p-4 transition-colors hover:bg-muted"
                >
                    <img
                        :src="word.screenshot"
                        :alt="word.word"
                        class="h-16 w-16 rounded object-cover"
                    />
                    <div class="flex-1">
                        <div class="flex items-start justify-between">
                            <div>
                                <h3 class="font-semibold text-foreground">
                                    {{ word.word }}
                                </h3>
                                <p class="mt-1 text-sm text-muted-foreground">
                                    {{ word.meaning }}
                                </p>
                            </div>
                            <Star
                                v-if="word.isFavorite"
                                class="h-5 w-5 fill-yellow-400 text-yellow-400"
                            />
                        </div>
                    </div>
                </div>
            </div>
        </Card>
    </div>
</template>
