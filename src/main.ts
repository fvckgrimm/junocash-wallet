import { createApp } from "vue";
import { createPinia } from "pinia"; // Import Pinia
import router from "./router";
import './style.css'
import App from "./App.vue";

//createApp(App).mount("#app");
const app = createApp(App);

app.use(createPinia());
app.use(router);

app.mount("#app");
