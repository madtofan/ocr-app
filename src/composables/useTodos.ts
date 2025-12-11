import { useQuery, useMutation, useQueryClient } from "@tanstack/vue-query";
import { invoke } from "@tauri-apps/api/core";

export interface Todo {
  id: number;
  title: string;
  status: "pending" | "done";
  created_at: string;
}

export function useTodos() {
  const queryClient = useQueryClient();

  // 1. READ (Call Rust 'get_todos')
  const query = useQuery({
    queryKey: ["todos"],
    queryFn: async () => {
      // invoke returns a Promise that resolves with the Rust return value
      return await invoke<Todo[]>("get_todos");
    },
  });

  // 2. CREATE (Call Rust 'add_todo')
  const addTodo = useMutation({
    mutationFn: async (title: string) => {
      await invoke("add_todo", { title });
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["todos"] });
    },
  });

  // 3. UPDATE (Call Rust 'toggle_todo')
  const toggleTodo = useMutation({
    mutationFn: async (todo: Todo) => {
      await invoke("toggle_todo", {
        id: todo.id,
        currentStatus: todo.status, // Note camelCase mapping if needed, but Rust expects snake_case args by default unless configured
      });
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["todos"] });
    },
  });

  // 4. DELETE (Call Rust 'delete_todo')
  const deleteTodo = useMutation({
    mutationFn: async (id: number) => {
      await invoke("delete_todo", { id });
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["todos"] });
    },
  });

  return {
    todos: query.data,
    isLoading: query.isLoading,
    error: query.error,
    addTodo,
    toggleTodo,
    deleteTodo,
  };
}
