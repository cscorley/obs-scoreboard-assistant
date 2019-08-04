<template>
  <div id="navigationView">
    <div>
      <b-navbar toggleable="lg" type="dark" variant="info">
        <b-navbar-brand href="#">Scoreboard Assistant</b-navbar-brand>

        <b-navbar-toggle target="nav-collapse"></b-navbar-toggle>

        <b-collapse id="nav-collapse" is-nav>
          <b-navbar-nav>
            <b-nav-item :to="{ path: `/names/${appKey}`}">Set Names</b-nav-item>
            <b-nav-item :to="{ path: `/edit/${appKey}`}">Edit Overlay</b-nav-item>
            <b-nav-item :to="{ path: `/overlay/${appKey}`}">Show Overlay</b-nav-item>
          </b-navbar-nav>

          <!-- Right aligned nav items -->
          <b-navbar-nav class="ml-auto"></b-navbar-nav>
        </b-collapse>
      </b-navbar>
    </div>
    <router-view></router-view>
  </div>
</template>

<script>
export default {
  name: "NavigationView",
  data: function() {
    return {
      appKey: "a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11"
    };
  }
};
</script>

<style scoped>
#editView {
  font-family: "Avenir", Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
