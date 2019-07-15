<template>
  <div id="editView">
    <p>
      <router-link to="/edit">Edit</router-link>&nbsp;
      <router-link :to="{ path: `/overlay/${appKey}`}">Overlay</router-link>
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
