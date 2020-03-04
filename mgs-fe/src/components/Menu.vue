<template>
  <aside class="menu is-sidebar-menu">
    <p class="menu-label">
      General
    </p>
    <ul class="menu-list">
      <li>
        <a
          v-on:click="navigate('/')"
          v-bind:class="{ active: location == '/' }"
        >
          Dashboard</a
        >
      </li>
      <li>
        <a
          v-on:click="navigate('/assets')"
          v-bind:class="{ active: location == '/assets' }"
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
    navigate: function(location: string) {
      store.dispatch("navigate", { router: this.$router, location });
    }
  },
  computed: {
    location: function() {
      return store.state.location;
    }
  }
});
</script>
