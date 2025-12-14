<script setup lang="ts">
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Progress } from "@/components/ui/progress";
import { Word } from "@/lib/types";
import {
    Check,
    ChevronRight,
    GraduationCap,
    RotateCcw,
    X,
} from "lucide-vue-next";
import { computed, ref } from "vue";

const WORD_TARGET = 10;

const words = ref<Word[]>([]);
const showAnswer = ref(false);
const updateShowAnswer = (newShowAnswer: boolean) => {
    showAnswer.value = newShowAnswer;
};

const todaySession = computed(() => {
    const isComplete = false;
    const correctCount = 0;
    const incorrectCount = 0;
    const currentIndex = correctCount + incorrectCount + 1;
    const currentWord = words.value[currentIndex % words.value.length];
    const progress = (correctCount + incorrectCount) / WORD_TARGET;

    return {
        correctCount,
        isComplete,
        incorrectCount,
        currentIndex,
        currentWord,
        progress,
    };
});

const handleRestart = () => {};
const handleNext = (isCorrect: boolean) => {};
</script>

<template>
    <div class="p-8">
        <div class="mb-8">
            <h1 class="text-3xl font-bold text-foreground">Study Mode</h1>
            <p class="mt-2 text-muted-foreground">
                Test your knowledge with flashcards
            </p>
        </div>

        <Card v-if="words.length === 0" class="p-12 text-center">
            <GraduationCap
                class="mx-auto mb-4 h-16 w-16 text-muted-foreground"
            />
            <p class="text-lg font-medium text-foreground">No words to study</p>
            <p class="mt-2 text-sm text-muted-foreground">
                Add some words to your vocabulary first
            </p>
        </Card>
        <Card
            v-else-if="todaySession.isComplete"
            class="mx-auto max-w-2xl p-12 text-center"
        >
            <Check class="mx-auto mb-4 h-16 w-16 text-green-500" />
            <h2 class="mb-2 text-2xl font-bold text-foreground">
                Study Session Complete!
            </h2>
            <p class="mb-6 text-muted-foreground">Great job studying today</p>

            <div class="mb-8 grid gap-4 md:grid-cols-2">
                <div class="rounded-lg bg-green-500/10 p-4">
                    <p class="text-3xl font-bold text-green-600">
                        {{ todaySession.correctCount }}
                    </p>
                    <p class="text-sm text-muted-foreground">Correct</p>
                </div>
                <div class="rounded-lg bg-red-500/10 p-4">
                    <p class="text-3xl font-bold text-red-600">
                        {{ todaySession.incorrectCount }}
                    </p>
                    <p class="text-sm text-muted-foreground">Incorrect</p>
                </div>
            </div>

            <Button :onclick="handleRestart" size="lg" class="gap-2">
                <RotateCcw class="h-4 w-4" />
                Study Again
            </Button>
        </Card>
        <div v-else class="mx-auto max-w-3xl">
            <!-- {/* Progress */} -->
            <div class="mb-6">
                <div class="mb-2 flex items-center justify-between text-sm">
                    <span class="text-muted-foreground">
                        {{
                            `Card ${todaySession.currentIndex} of ${words.length}`
                        }}
                    </span>
                    <span class="text-muted-foreground">
                        {{
                            `${todaySession.correctCount} correct • ${todaySession.incorrectCount} incorrect`
                        }}
                    </span>
                </div>
                <Progress :value="todaySession.progress" class="h-2" />
            </div>

            <!-- {/* Flashcard */} -->
            <Card class="mb-6 overflow-hidden">
                <img
                    :src="todaySession.currentWord.screenshot"
                    :alt="todaySession.currentWord.word"
                    class="h-64 w-full object-cover"
                />
                <div class="p-8">
                    <div class="mb-6 text-center">
                        <h2 class="mb-4 text-4xl font-bold text-foreground">
                            {{ todaySession.currentWord.word }}
                        </h2>

                        <div
                            v-if="showAnswer"
                            class="mt-6 rounded-lg bg-muted p-6"
                        >
                            <p class="text-lg text-foreground">
                                {{ todaySession.currentWord.meaning }}
                            </p>

                            <p class="mt-2 text-sm text-muted-foreground">
                                {{ todaySession.currentWord.pronunciation }}
                            </p>

                            <div
                                v-if="
                                    todaySession.currentWord.examples &&
                                    todaySession.currentWord.examples.length > 0
                                "
                                class="mt-4 text-left"
                            >
                                <p
                                    class="mb-2 text-sm font-medium text-foreground"
                                >
                                    Examples:
                                </p>
                                <p
                                    v-for="(example, index) in todaySession
                                        .currentWord.examples"
                                    :key="index"
                                    class="text-sm italic text-muted-foreground"
                                >
                                    {{ `"${example}"` }}
                                </p>
                            </div>
                        </div>
                        <div v-else class="mt-6">
                            <p class="text-muted-foreground">
                                Think about the meaning, then reveal the answer
                            </p>
                        </div>
                    </div>
                </div>
            </Card>

            <!-- {/* Actions */} -->
            <div class="flex gap-4">
                {!showAnswer ? (
                <Button
                    v-if="!showAnswer"
                    :onclick="() => updateShowAnswer(true)"
                    size="lg"
                    class="flex-1 gap-2"
                >
                    Show Answer
                    <ChevronRight class="h-4 w-4" />
                </Button>
                <template v-else>
                    <Button
                        :onclick="() => handleNext(false)"
                        variant="outline"
                        size="lg"
                        class="flex-1 gap-2 border-red-500 text-red-500 hover:bg-red-50"
                    >
                        <X class="h-5 w-5" />
                        Incorrect
                    </Button>
                    <Button
                        :onclick="() => handleNext(true)"
                        size="lg"
                        class="flex-1 gap-2 bg-green-500 hover:bg-green-600"
                    >
                        <Check class="h-5 w-5" />
                        Correct
                    </Button>
                </template>
            </div>
        </div>
    </div>
</template>
