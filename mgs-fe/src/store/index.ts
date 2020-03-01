import Vue from "vue";
import Vuex from "vuex";
import Axios from "axios";

import AssetModel from "@/models/AssetModel";

Vue.use(Vuex);

export default new Vuex.Store({
  state: {
    assets: Array<AssetModel>()
  },
  mutations: {
    loadAssets(state, assetModels: Array<AssetModel>) {
      state.assets = assetModels;
    }
  },
  actions: {
    loadAssets({ commit }) {
      Axios.request<Array<AssetModel>>({
        url: "http://localhost:8000/assets"
      }).then(data => {
        commit("loadAssets", data.data);
      });
    }
  },
  modules: {}
});
