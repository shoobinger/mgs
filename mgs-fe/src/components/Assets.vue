<template>
  <div class="container is-widescreen">
    <table class="table is-fullwidth">
      <thead>
        <tr>
          <th>ID</th>
          <th>Name</th>
          <th>Description</th>
          <th>Added</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="asset in assets" :key="asset.id">
          <td>{{ asset.id }}</td>
          <td>{{ asset.name }}</td>
          <td>{{ asset.description }}</td>
          <td>{{ asset.createdAt }}</td>
        </tr>
      </tbody>
    </table>
    <button
      class="button is-light is-fullwidth"
      v-if="hasMore"
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

export default Vue.extend({
  methods: {
    loadNext: function() {
      this.$store.dispatch("loadAssets", this.nextOffset);
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
    nextOffset() {
      return (
        this.$store.state.pagination.offset + this.$store.state.pagination.limit
      );
    }
  }
});
</script>
