<script setup lang="ts">
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from "@/components/ui/select";
import { pagePropertiesKey } from "@/lib/keys";
import { AppConfig } from "@/lib/types";
import { Globe, Keyboard, LayoutGrid, Save } from "lucide-vue-next";
import { inject, ref } from "vue";

const config = ref<AppConfig>({
    dictionaryProvider: "free-dictionary",
    sourceLanguage: "",
    targetLanguage: "",
    ocrBounds: {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    },
    captureShortcut: "",
    autoSave: false,
    theme: "light",
});

const updateConfig = (newConfig: Partial<AppConfig>) => {
    config.value = { ...config.value, ...newConfig };
};

const handleSave = () => {
    // TODO - convert this page into a form
    console.log("Saving this config");
};

const { updateHeader } = inject(pagePropertiesKey, {
    header: "",
    updateHeader: (_: string) => {},
});

updateHeader("Settings Page");
</script>

<template>
    <div class="p-8">
        <div class="mb-8">
            <h1 class="text-3xl font-bold text-foreground">Settings</h1>
            <p class="mt-2 text-muted-foreground">
                Configure your dictionary, OCR bounds, and keyboard shortcuts
            </p>
        </div>

        <div class="max-w-3xl space-y-6">
            <!-- {/* Dictionary Settings */} -->
            <Card class="p-6">
                <div class="mb-4 flex items-center gap-3">
                    <Globe class="h-5 w-5 text-muted-foreground" />
                    <h2 class="text-lg font-semibold text-foreground">
                        Dictionary Settings
                    </h2>
                </div>

                <div class="space-y-4">
                    <div>
                        <Label htmlFor="dictionary">Dictionary Provider</Label>
                        <Select
                            :value="config.dictionaryProvider"
                            :onchange="
                                (
                                    dictionaryProvider: AppConfig['dictionaryProvider'],
                                ) => updateConfig({ dictionaryProvider })
                            "
                        >
                            <SelectTrigger id="dictionary" class="mt-2">
                                <SelectValue />
                            </SelectTrigger>
                            <SelectContent>
                                <SelectItem value="free-dictionary"
                                    >Free Dictionary API</SelectItem
                                >
                                <SelectItem value="google-translate"
                                    >Google Translate</SelectItem
                                >
                                <SelectItem value="custom"
                                    >Custom Dictionary</SelectItem
                                >
                            </SelectContent>
                        </Select>
                    </div>

                    <div v-if="config.dictionaryProvider === 'custom'">
                        <Label for="customUrl">Custom Dictionary URL</Label>
                        <Input
                            id="customUrl"
                            :value="config.customDictionaryUrl || ''"
                            :onchange="
                                (
                                    customDictionaryUrl: AppConfig['customDictionaryUrl'],
                                ) => updateConfig({ customDictionaryUrl })
                            "
                            placeholder="https://api.example.com/dictionary"
                            class="mt-2"
                        />
                    </div>

                    <div class="grid gap-4 md:grid-cols-2">
                        <div>
                            <Label for="sourceLang">Source Language</Label>
                            <Input
                                id="sourceLang"
                                :value="config.sourceLanguage"
                                :onchange="
                                    (
                                        sourceLanguage: AppConfig['sourceLanguage'],
                                    ) => updateConfig({ sourceLanguage })
                                "
                                placeholder="en"
                                class="mt-2"
                            />
                        </div>

                        <div>
                            <Label for="targetLang">Target Language</Label>
                            <Input
                                id="targetLang"
                                :value="config.targetLanguage"
                                :onchange="
                                    (
                                        targetLanguage: AppConfig['targetLanguage'],
                                    ) => updateConfig({ targetLanguage })
                                "
                                placeholder="es"
                                class="mt-2"
                            />
                        </div>
                    </div>
                </div>
            </Card>

            <!-- {/* OCR Bounds Settings */} -->
            <Card class="p-6">
                <div class="mb-4 flex items-center gap-3">
                    <LayoutGrid class="h-5 w-5 text-muted-foreground" />
                    <h2 class="text-lg font-semibold text-foreground">
                        OCR Capture Bounds
                    </h2>
                </div>

                <div class="space-y-4">
                    <p class="text-sm text-muted-foreground">
                        Define the screen region where OCR should capture text
                    </p>

                    <div class="grid gap-4 md:grid-cols-2">
                        <div>
                            <Label htmlFor="x">X Position</Label>
                            <Input
                                id="x"
                                type="number"
                                :value="config.ocrBounds.x"
                                :onchange="
                                    (x: number) =>
                                        updateConfig({
                                            ocrBounds: {
                                                ...config.ocrBounds,
                                                x,
                                            },
                                        })
                                "
                                class="mt-2"
                            />
                        </div>

                        <div>
                            <Label for="y">Y Position</Label>
                            <Input
                                id="y"
                                type="number"
                                :value="config.ocrBounds.y"
                                :onchange="
                                    (y: number) =>
                                        updateConfig({
                                            ocrBounds: {
                                                ...config.ocrBounds,
                                                y,
                                            },
                                        })
                                "
                                class="mt-2"
                            />
                        </div>

                        <div>
                            <Label htmlFor="width">Width</Label>
                            <Input
                                id="width"
                                type="number"
                                :value="config.ocrBounds.width"
                                :onchange="
                                    (width: number) =>
                                        updateConfig({
                                            ocrBounds: {
                                                ...config.ocrBounds,
                                                width,
                                            },
                                        })
                                "
                                class="mt-2"
                            />
                        </div>

                        <div>
                            <Label htmlFor="height">Height</Label>
                            <Input
                                id="height"
                                type="number"
                                :value="config.ocrBounds.height"
                                :onchange="
                                    (height: number) =>
                                        updateConfig({
                                            ocrBounds: {
                                                ...config.ocrBounds,
                                                height,
                                            },
                                        })
                                "
                                class="mt-2"
                            />
                        </div>
                    </div>

                    <div class="rounded-lg border border-border bg-muted p-4">
                        <p class="text-sm font-medium text-foreground">
                            Preview
                        </p>
                        <p class="mt-1 text-xs text-muted-foreground">
                            Region: {{ config.ocrBounds.x }},
                            {{ config.ocrBounds.y }} | Size:
                            {{ config.ocrBounds.width }}x
                            {{ config.ocrBounds.height }}px
                        </p>
                    </div>
                </div>
            </Card>

            <!-- {/* Keyboard Shortcut */} -->
            <Card class="p-6">
                <div class="mb-4 flex items-center gap-3">
                    <Keyboard class="h-5 w-5 text-muted-foreground" />
                    <h2 class="text-lg font-semibold text-foreground">
                        Keyboard Shortcut
                    </h2>
                </div>

                <div class="space-y-4">
                    <div>
                        <Label for="shortcut">Capture Shortcut</Label>
                        <Input
                            id="shortcut"
                            :value="config.captureShortcut"
                            :onchange="
                                (captureShortcut: string) =>
                                    updateConfig({ captureShortcut })
                            "
                            placeholder="Ctrl+Shift+C"
                            class="mt-2"
                        />
                        <p class="mt-2 text-xs text-muted-foreground">
                            Use format: Ctrl+Shift+C, Alt+C, etc.
                        </p>
                    </div>
                </div>
            </Card>

            <!-- {/* Save Button */} -->
            <div class="flex justify-end gap-4">
                <Button :onclick="handleSave" size="lg" class="gap-2">
                    <Save class="h-4 w-4" />
                    Save Settings
                </Button>
            </div>
        </div>
    </div>
</template>
