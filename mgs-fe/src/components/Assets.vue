<template>
  <div class="container is-widescreen">
    <div class="level">
      <div class="level-left">
        <button class="button is-light level-item" v-on:click="addAsset">
          Add asset
        </button>
      </div>
    </div>
    <div v-for="asset in assets" :key="asset.id" class="level">
      <div class="level-left">
        <div class="level-item is-size-5">{{ asset.name }}</div>
        <div class="level-item is-size-6" v-if="asset.description">
          {{ asset.description }}
        </div>
        <div class="level-item"><AssetType v-bind:name="asset.type" /></div>
      </div>
      <div class="level-right">
        <div class="level-item">{{ asset.quantity }}</div>
        <div class="level-item">{{ asset.value }}</div>
      </div>
    </div>
    <button
      class="button is-light is-fullwidth"
      v-if="hasMore"
      v-bind:class="{ 'is-loading': loading }"
      v-on:click="loadNext"
    >
      Load more
    </button>
  </div>
</template>

<script lang="ts">
import Vue from "vue";
import dayjs from "dayjs";
import store from "@/store";
import AssetType from "@/components/AssetType.vue";

export default Vue.extend({
  components: { AssetType },
  methods: {
    loadNext() {
      this.$store.dispatch("loadAssets", this.nextOffset);
    },
    addAsset() {
      this.$store.dispatch("navigate", {
        router: this.$router,
        location: "/assets/add"
      });
    }
  },
  created() {
    this.$store.dispatch("loadAssets", 0);
  },
  beforeDestroy() {
    this.$store.dispatch("unloadAssets");
  },
  computed: {
    assets() {
      return store.state.assets.map(am => {
        return {
          ...am,
          createdAt: dayjs(am.createdAt).format("MM-DD-YYYY")
        };
      });
    },
    hasMore() {
      return this.$store.state.pagination.hasMore;
    },
    loading() {
      return this.$store.state.loading;
    },
    nextOffset() {
      return (
        this.$store.state.pagination.offset + this.$store.state.pagination.limit
      );
    }
  }
});
</script>
