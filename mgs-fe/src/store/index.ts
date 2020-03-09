import Vue from "vue";
import Vuex from "vuex";
import Axios from "axios";
import Router, { RawLocation } from "vue-router";

import AssetModel from "@/models/AssetModel";

Vue.use(Vuex);

class Pagination {
  constructor(
    readonly limit: number,
    readonly offset: number,
    readonly hasMore: boolean
  ) {}
}

interface State {
  location: RawLocation;
  assets: Array<AssetModel>;
  loading: boolean;
  pagination: Pagination;
}

const initialState: State = {
  location: "/",
  assets: Array<AssetModel>(),
  loading: false,
  pagination: new Pagination(0, 0, false)
};

class NavigatePayload {
  constructor(readonly router: Router, readonly location: RawLocation) {}
}

interface Page<T> {
  elements: Array<T>;
  limit: number;
  offset: number;
  hasMore: boolean;
}

export default new Vuex.Store<State>({
  state: initialState,
  mutations: {
    navigate(state: State, location: string) {
      state.location = location;
    },
    loadAssets(state, assetsPage: Page<AssetModel>) {
      state.assets = state.assets.concat(assetsPage.elements);

      const hasMore = assetsPage.elements.length == assetsPage.limit;
      state.pagination = new Pagination(
        assetsPage.limit,
        assetsPage.offset,
        hasMore
      );
    },
    unloadAssets(state) {
      state.assets = [];
      state.pagination = new Pagination(0, 0, false);
    },
    setLoading(state, loading: boolean) {
      state.loading = loading;
    }
  },
  actions: {
    setInitialPath({ commit }, location) {
      commit("navigate", location);
    },
    navigate({ commit }, payload: NavigatePayload) {
      commit("navigate", payload.location);
      payload.router.push(payload.location);
    },
    loadAssets({ commit }, offset: number) {
      commit("setLoading", true);

      Axios.request<Page<AssetModel>>({
        url: `http://localhost:8000/assets?offset=${offset}&limit=2`
      })
        .then(
          data => {
            commit("loadAssets", data.data);
          },
          reason => {
            commit("setError", "error occurred"); //TODO handle error
          }
        )
        .finally(() => {
          commit("setLoading", false);
        });
    },
    unloadAssets({ commit }) {
      commit("unloadAssets");
    }
  },
  modules: {}
});
