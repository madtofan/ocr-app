export interface Word {
  id: string;
  word: string;
  meaning: string;
  screenshot: string; // base64 or URL
  translatedText: string;
  sourceLanguage: string;
  targetLanguage: string;
  createdAt: string;
  isFavorite: boolean;
  groupIds: string[];
  tags: string[];
  pronunciation?: string;
  examples?: string[];
  notes?: string;
}

export interface WordGroup {
  id: string;
  name: string;
  description?: string;
  color: string;
  createdAt: string;
  wordCount: number;
}

export interface AppConfig {
  dictionaryProvider: "free-dictionary" | "google-translate" | "custom";
  customDictionaryUrl?: string;
  sourceLanguage: string;
  targetLanguage: string;
  ocrBounds: {
    x: number;
    y: number;
    width: number;
    height: number;
  };
  captureShortcut: string;
  autoSave: boolean;
  theme: "light" | "dark" | "system";
}

export interface UserStats {
  totalWords: number;
  wordsThisWeek: number;
  wordsThisMonth: number;
  favoriteCount: number;
  groupCount: number;
  studySessions: number;
  lastStudyDate?: string;
}
