<template>
  <aside class="menu is-sidebar-menu">
    <p class="menu-label">
      General
    </p>
    <ul class="menu-list">
      <li>
        <a
          v-on:click="navigate('/')"
          v-bind:class="{ 'is-active': currentLocation == '/' }"
        >
          Dashboard</a
        >
      </li>
      <li>
        <a
          v-on:click="navigate('/assets')"
          v-bind:class="{ 'is-active': currentLocation == '/assets' }"
          >Assets</a
        >
      </li>
    </ul>
  </aside>
</template>

<script lang="ts">
import Vue from "vue";
import store from "@/store";

export default Vue.extend({
  methods: {
    navigate(location: string) {
      if (location != this.currentLocation) {
        store.dispatch("navigate", { router: this.$router, location });
      }
    }
  },
  computed: {
    currentLocation() {
      return store.state.location;
    }
  }
});
</script>
