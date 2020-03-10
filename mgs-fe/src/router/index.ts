import Vue from "vue";
import VueRouter, { RouteConfig } from "vue-router";
import AssetsPage from "@/views/AssetsPage.vue";

Vue.use(VueRouter);

const routes: RouteConfig[] = [
  {
    path: "/",
    name: "Dashboard",
    component: () =>
      import(/* webpackChunkName: "dashboard" */ "../views/DashboardPage.vue")
  },
  {
    path: "/assets",
    component: AssetsPage,
    children: [
      {
        path: "",
        name: "Assets",
        component: () =>
          import(/* webpackChunkName: "assets" */ "../components/Assets.vue")
      },
      {
        path: "add",
        name: "Add a new asset",
        component: () =>
          import(
            /* webpackChunkName: "add_asset" */ "../components/AddAsset.vue"
          )
      }
    ]
  }
];

const router = new VueRouter({
  mode: "history",
  base: process.env.BASE_URL,
  routes
});

export default router;
