import { createWebHistory, createRouter } from "vue-router";
import ScreenshotPage from "./views/ScreenshotPage.vue";
import SettingPage from "./views/SettingPage.vue";
import NotFound from "./views/NotFound.vue";
import TodoPage from "./views/TodoPage.vue";
import OverlayPage from "./views/OverlayPage.vue";
import AppLayout from "./components/AppLayout.vue";

const routes = [
  {
    path: "/",
    name: "App Layout",
    component: AppLayout,
    children: [
      {
        path: "",
        name: "Dashboard",
        component: ScreenshotPage,
      },
      {
        path: "/todo",
        name: "Todos",
        component: TodoPage,
      },
      {
        path: "/settings",
        name: "Settings",
        component: SettingPage,
      },
    ],
  },
  {
    path: "/overlay",
    name: "Overlay",
    component: OverlayPage,
  },
  {
    path: "/:catchAll(.*)",
    name: "Not Found",
    component: NotFound,
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
