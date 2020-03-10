<template>
  <div class="container is-widescreen">
    <div class="field">
      <label class="label">Type</label>
      <div class="control">
        <div class="select">
          <select v-model="asset.type">
            <option v-for="assetType in assetTypes" v-bind:key="assetType">{{
              assetType
            }}</option>
          </select>
        </div>
      </div>
    </div>
    <div class="field">
      <label class="label">Name</label>
      <div class="control">
        <input class="input" type="text" v-model="asset.name" />
      </div>
    </div>
    <div class="field">
      <label class="label">Description</label>
      <div class="control">
        <input class="input" type="text" v-model="asset.description" />
      </div>
    </div>
    <div class="control">
      <button
        class="button is-primary"
        v-on:click="submit"
        v-bind:class="{ 'is-loading': loading }"
      >
        Add
      </button>
    </div>
  </div>
</template>

<script lang="ts">
import Vue from "vue";
import dayjs from "dayjs";
import store from "@/store";
// import AssetType from "@/components/AssetType.vue";
import { AssetType, assetTypes } from "@/service/enums";

export default Vue.extend({
  data() {
    return {
      loading: false,
      asset: {
        name: null,
        description: null,
        type: null
      }
    };
  },
  methods: {
    submit() {
      this.loading = true;

      this.$store
        .dispatch("addAsset", this.asset)
        .then(() => {
          this.$emit("assetAdded");
        })
        .finally(() => {
          this.loading = false;
        });
    }
  },
  computed: {
    assetTypes: () => assetTypes
  }
});
</script>
