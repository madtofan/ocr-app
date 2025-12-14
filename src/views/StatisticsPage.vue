<script setup lang="ts">
import { Card } from "@/components/ui/card";
import { UserStats } from "@/lib/types";
import { BarChart3, Calendar, Target, TrendingUp } from "lucide-vue-next";
import { computed } from "vue";

interface DayData {
    day: string;
    count: number;
}

const weeklyData = computed<{ data: DayData[]; maxCount: number }>(() => {
    return {
        data: [
            {
                day: "Mon",
                count: 1,
            },
            {
                day: "Tue",
                count: 2,
            },
            {
                day: "Wed",
                count: 1,
            },
            {
                day: "Thu",
                count: 5,
            },
            {
                day: "Fri",
                count: 0,
            },
            {
                day: "Sat",
                count: 0,
            },
            {
                day: "Sun",
                count: 9,
            },
        ],
        maxCount: 9,
    };
});

const stats = computed<UserStats>(() => {
    return {
        totalWords: 0,
        wordsThisWeek: 0,
        wordsThisMonth: 0,
        favoriteCount: 0,
        groupCount: 0,
        studySessions: 0,
        lastStudyDate: new Date().toDateString(),
    };
});
</script>

<template>
    <div class="p-8">
        <div class="mb-8">
            <h1 class="text-3xl font-bold text-foreground">Statistics</h1>
            <p class="mt-2 text-muted-foreground">
                Track your learning progress and achievements
            </p>
        </div>

        <!-- {/* Stats Overview */} -->
        <div class="mb-8 grid gap-6 md:grid-cols-2 lg:grid-cols-4">
            <Card class="p-6">
                <div class="flex items-center gap-4">
                    <div class="rounded-full bg-primary/10 p-3">
                        <Target class="h-6 w-6 text-primary" />
                    </div>
                    <div>
                        <p class="text-sm text-muted-foreground">Total Words</p>
                        <p class="text-2xl font-bold text-foreground">
                            {{ stats.totalWords }}
                        </p>
                    </div>
                </div>
            </Card>

            <Card class="p-6">
                <div class="flex items-center gap-4">
                    <div class="rounded-full bg-green-500/10 p-3">
                        <TrendingUp class="h-6 w-6 text-green-500" />
                    </div>
                    <div>
                        <p class="text-sm text-muted-foreground">This Week</p>
                        <p class="text-2xl font-bold text-foreground">
                            {{ stats.wordsThisWeek }}
                        </p>
                    </div>
                </div>
            </Card>

            <Card class="p-6">
                <div class="flex items-center gap-4">
                    <div class="rounded-full bg-blue-500/10 p-3">
                        <Calendar class="h-6 w-6 text-blue-500" />
                    </div>
                    <div>
                        <p class="text-sm text-muted-foreground">This Month</p>
                        <p class="text-2xl font-bold text-foreground">
                            {{ stats.wordsThisMonth }}
                        </p>
                    </div>
                </div>
            </Card>

            <Card class="p-6">
                <div class="flex items-center gap-4">
                    <div class="rounded-full bg-purple-500/10 p-3">
                        <BarChart3 class="h-6 w-6 text-purple-500" />
                    </div>
                    <div>
                        <p class="text-sm text-muted-foreground">
                            Study Sessions
                        </p>
                        <p class="text-2xl font-bold text-foreground">
                            {{ stats.studySessions }}
                        </p>
                    </div>
                </div>
            </Card>
        </div>

        <!-- {/* Weekly Activity Chart */} -->
        <Card class="p-6">
            <h2 class="mb-6 text-xl font-semibold text-foreground">
                Weekly Activity
            </h2>

            <div class="flex items-end justify-between gap-4 h-64">
                <div
                    v-for="(data, index) in weeklyData.data"
                    :key="index"
                    class="flex flex-1 flex-col items-center gap-2"
                >
                    <div class="relative w-full flex-1">
                        <div
                            class="absolute bottom-0 w-full rounded-t-lg bg-primary transition-all"
                            :class="`h-[${(data.count / weeklyData.maxCount) * 100}%]`"
                        />
                    </div>
                    <div class="text-center">
                        <p class="text-sm font-medium text-foreground">
                            {{ data.count }}
                        </p>
                        <p class="text-xs text-muted-foreground">
                            {{ data.day }}
                        </p>
                    </div>
                </div>
            </div>
        </Card>

        <!-- {/* Achievement Cards */} -->
        <div class="mt-8 grid gap-6 md:grid-cols-3">
            <Card class="p-6">
                <div class="mb-4 text-4xl">🎯</div>
                <h3 class="mb-2 text-lg font-semibold text-foreground">
                    Getting Started
                </h3>
                <p class="text-sm text-muted-foreground">
                    {{
                        stats.totalWords >= 10
                            ? "Unlocked!"
                            : `Add ${10 - stats.totalWords} more words`
                    }}
                </p>
                <div class="mt-3 h-2 w-full rounded-full bg-muted">
                    <div
                        class="h-full rounded-full bg-primary transition-all"
                        :class="`w-[${Math.min((stats.totalWords / 10) * 100, 100)}%]`"
                    />
                </div>
            </Card>

            <Card class="p-6">
                <div class="mb-4 text-4xl">🔥</div>
                <h3 class="mb-2 text-lg font-semibold text-foreground">
                    Building Momentum
                </h3>
                <p class="text-sm text-muted-foreground">
                    {{
                        stats.totalWords >= 50
                            ? "Unlocked!"
                            : `Add ${50 - stats.totalWords} more words`
                    }}
                </p>
                <div class="mt-3 h-2 w-full rounded-full bg-muted">
                    <div
                        class="h-full rounded-full bg-primary transition-all"
                        :class="`w-[${Math.min((stats.totalWords / 50) * 100, 100)}%]`"
                    />
                </div>
            </Card>

            <Card class="p-6">
                <div class="mb-4 text-4xl">🏆</div>
                <h3 class="mb-2 text-lg font-semibold text-foreground">
                    Word Master
                </h3>
                <p class="text-sm text-muted-foreground">
                    {{
                        stats.totalWords >= 100
                            ? "Unlocked!"
                            : `Add ${100 - stats.totalWords} more words`
                    }}
                </p>
                <div class="mt-3 h-2 w-full rounded-full bg-muted">
                    <div
                        class="h-full rounded-full bg-primary transition-all"
                        :class="`w-[${Math.min((stats.totalWords / 100) * 100, 100)}%]`"
                    />
                </div>
            </Card>
        </div>
    </div>
</template>
