<template>
  <div id="editView">
    <h1>Hello App!</h1>
    <p>
      <!-- use router-link component for navigation. -->
      <!-- specify the link by passing the `to` prop. -->
      <!-- `<router-link>` will be rendered as an `<a>` tag by default -->
      <router-link to="/edit">Go to Foo</router-link>
      <br />
      <router-link to="/overlay">Go to Bar</router-link>
    </p>
    <input id="appKeyInput" v-model="appKey" />
    <PlayerEdit
      :id="0"
      :app-key="appKey"
      :initial-name="defaultName"
      :initial-score="defaultScore"
    />
    <PlayerEdit
      :id="1"
      :app-key="appKey"
      :initial-name="defaultName"
      :initial-score="defaultScore"
    />
  </div>
</template>

<script>
import PlayerEdit from "./PlayerEdit.vue";

export default {
  name: "EditView",
  data: function() {
    return {
      appKey: "a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11",
      defaultName: "Unknown",
      defaultScore: 0,
      message: "butts butts butts",
      otherThing: 420
    };
  },
  components: {
    PlayerEdit
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
