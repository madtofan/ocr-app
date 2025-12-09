import { VueQueryPlugin } from "@tanstack/vue-query";
import { createApp } from "vue";
import App from "./App.vue";
// Import the Tailwind CSS input file
import "./assets/main.css";
import router from "./routes";

const app = createApp(App);
app.use(VueQueryPlugin).use(router);
app.mount("#app");
