import Vue from "vue";
import Vuex from "vuex";
import Axios from "axios";
import Router, { RawLocation } from "vue-router";

import AssetModel from "@/models/AssetModel";

Vue.use(Vuex);

interface State {
  location: RawLocation;
  assets: Array<AssetModel>;
}

const initialState: State = {
  location: "/",
  assets: Array<AssetModel>()
};

interface NavigatePayload {
  router: Router;
  location: RawLocation;
}

export default new Vuex.Store<State>({
  state: initialState,
  mutations: {
    navigate(state: State, location: string) {
      state.location = location;
    },
    loadAssets(state, assetModels: Array<AssetModel>) {
      state.assets = assetModels;
    }
  },
  actions: {
    navigate({ commit }, payload: NavigatePayload) {
      commit("navigate", payload.location);
      payload.router.push(payload.location);
    },
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
