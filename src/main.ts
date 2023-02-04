import { createApp } from "vue";
import "highlight.js/styles/a11y-dark.css";
import "./main.css";
import App from "./App.vue";
import router from "./router";

createApp(App).use(router).mount("#app");
