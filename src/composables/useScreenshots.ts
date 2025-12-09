import { useQuery, useQueryClient } from "@tanstack/vue-query";
import { readDir, BaseDirectory } from "@tauri-apps/plugin-fs";
import { convertFileSrc } from "@tauri-apps/api/core";

export interface Screenshot {
  name: string;
  path: string;
  url: string;
}

// Inside your function:
export function useScreenshots() {
  const queryClient = useQueryClient();

  const query = useQuery({
    queryKey: ["screenshots"],
    queryFn: async (): Promise<Screenshot[]> => {
      // 1. Read files from the user's Picture directory
      // Ensure your Rust code saves here!
      const entries = await readDir("", { baseDir: BaseDirectory.Picture });

      // 2. Filter and Map
      const images = entries
        .filter(
          (entry) => entry.isFile && /\.(png|jpg|jpeg)$/i.test(entry.name),
        )
        .map((entry) => ({
          name: entry.name,
          path: entry.name, // In a real app, you might want the full path
          // 3. Convert to a WebView-safe URL
          url: convertFileSrc(entry.name),
        }));

      // 4. Sort (Optional: assumes timestamp naming like screenshot_2024...)
      return images.sort((a, b) => b.name.localeCompare(a.name));
    },
    // Refetch when window creates focus (optional but nice for desktop apps)
    refetchOnWindowFocus: true,
  });

  return {
    ...query,
    refresh: () => queryClient.invalidateQueries({ queryKey: ["screenshots"] }),
  };
}
