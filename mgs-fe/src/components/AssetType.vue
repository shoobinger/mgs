<template>
  <span class="tag is-info is-light">{{ name }}</span>
</template>

<script lang="ts">
import Vue from "vue";
// import { AssetType } from "@/service/enums";
export default Vue.extend({
  props: {
    name: {
      type: String,
      required: true
    }
  }
  // computed: {
  // assetLabel: function(): string {
  // return AssetType[this.id];
  // }
  // }
});
</script>
