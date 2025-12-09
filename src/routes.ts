import { createWebHistory, createRouter } from "vue-router";
import ScreenshotPage from "./views/ScreenshotPage.vue";
import SettingPage from "./views/SettingPage.vue";

const routes = [
  {
    path: "/",
    name: "Home",
    component: ScreenshotPage,
  },
  {
    path: "/settings",
    name: "Settings",
    component: SettingPage,
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
