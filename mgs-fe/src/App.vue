<template>
  <div
    class="is-flex has-direction-column is-viewport-height"
    id="main-container"
  >
    <nav class="navbar" role="navigation">
      <div class="navbar-brand">
        <router-link class="navbar-item" to="/">MGS</router-link>
      </div>
    </nav>
    <div class="columns is-fullheight" id="app">
      <div class="column is-2 is-hidden-mobile">
        <Menu />
      </div>
      <div class="column is-main-content">
        <router-view />
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import Menu from "@/components/Menu.vue";
import Vue from "vue";

export default Vue.extend({
  name: "App",
  components: {
    Menu
  },
  created() {
    //TODO using window here because it's not clear why $router contains "/" instead of actual location
    //occurs when using Webpack lazy-loading
    this.$store.dispatch("setInitialPath", window.location.pathname);
  }
});
</script>

<style lang="scss" scoped>
.is-viewport-height {
  height: 100vh;
}

.has-direction-column {
  flex-direction: column;
}

.align-content-stretch {
  align-content: stretch;
}

.is-sidebar-menu {
  padding: 1rem;
}

.columns {
  &.is-fullheight {
    flex: 1;

    .column {
      overflow-y: auto;
    }
  }
}
</style>
