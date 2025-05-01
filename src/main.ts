import { createApp } from "vue";
import App from "./App.vue";
import { ModRouter } from "@/router";

createApp(App).use(ModRouter).mount("body");

// #region 挂载后操作
// 加载样式
import "@/styles/main.css";
import "@/styles/tailwind.css";
// #endregion
