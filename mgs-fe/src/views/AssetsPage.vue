<template>
  <div>
    <nav class="level">
      <div class="level-left">
        <button class="button is-light level-item">Back</button>
        <div class="title is-5 level-item">{{ name }}</div>
      </div>
    </nav>
    <router-view @assetAdded="assetAdded"></router-view>
  </div>
</template>

<script lang="ts">
import Vue from "vue";
export default Vue.extend({
  name: "AssetsPage",
  methods: {
    assetAdded() {
      this.$store.dispatch("navigate", {
        router: this.$router,
        location: "/assets"
      });
    }
  },
  computed: {
    name() {
      return this.$router.currentRoute.name;
    }
  }
});
</script>
