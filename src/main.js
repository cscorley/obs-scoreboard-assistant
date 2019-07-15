import Vue from "vue";
import VueRouter from "vue-router";
import App from "./App.vue";
import Overlay from "./components/Overlay.vue";
import EditView from "./components/EditView.vue";

Vue.config.productionTip = false;

Vue.use(VueRouter);

const routes = [
  { path: "/edit", component: EditView },
  { path: "/overlay/:appKey", component: Overlay }
];

const router = new VueRouter({ routes });

new Vue({
  render: h => h(App),
  router: router
}).$mount("#app");
