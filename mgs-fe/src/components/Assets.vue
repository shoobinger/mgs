<template>
  <div class="assets">
    <ul class="asset-list">
      <li v-for="asset in assets" :key="asset.id">
        {{ asset.name }} {{ asset.created_at }}
      </li>
    </ul>
  </div>
</template>

<script lang="ts">
import Vue from "vue";
import store from "@/store";

export default Vue.extend({
  created: () => store.dispatch("loadAssets"),
  computed: {
    assets: () => store.state.assets
  }
});
</script>
