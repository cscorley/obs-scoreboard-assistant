import "@babel/polyfill";
import "mutationobserver-shim";
import Vue from "vue";
import VueRouter from "vue-router";
import App from "./App.vue";
import BootstrapVue from "bootstrap-vue";
import "bootstrap/dist/css/bootstrap.min.css";
import "bootstrap-vue/dist/bootstrap-vue.css";

import NavigationView from "./components/NavigationView.vue";
import EditView from "./components/EditView.vue";
import SetNames from "./components/SetNames.vue";
import Overlay from "./components/Overlay.vue";

Vue.config.productionTip = false;

Vue.use(BootstrapVue);
Vue.use(VueRouter);

const routes = [
  {
    path: "/",
    component: NavigationView,
    children: [
      { path: "/edit/:appKey", component: EditView },
      { path: "/names/:appKey", component: SetNames }
    ]
  },
  { path: "/overlay/:appKey", component: Overlay }
];

const router = new VueRouter({ routes });

new Vue({
  render: h => h(App),
  router: router
}).$mount("#app");
