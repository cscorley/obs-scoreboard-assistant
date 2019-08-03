
<template>
  <div id="overlay">
      <div class="clip" id="left-name">{{ players[0].name }}</div>
      <div class="clip" id="right-name">{{ players[1].name }}</div>
      <div id="left-score">{{ players[0].score }}</div>
      <div id="right-score">{{ players[1].score }}</div>
  </div>
</template>

<script>
import axios from "axios";
import { setInterval } from "timers";

export default {
  name: "Overlay",
  data: function() {
    return {
      players: [{ name: "Unknown", score: 0 }, { name: "Unknown", score: 0 }]
    };
  },
  computed: {
    appKey: function() {
      return this.$route.params.appKey;
    }
  },
  created() {
    this.performUpdate();
    setInterval(this.performUpdate, 5000);
  },
  methods: {
    performUpdate() {
      [0, 1].forEach(element => {
        var endpoint = `/api/${this.appKey}/player/${element}`;
        axios
          .get(endpoint)
          .then(response => {
            if (response.data.name !== undefined) {
              this.players[element].name = response.data.name;
              this.players[element].score = response.data.score;
            }
          })
          .catch();
      });
    }
  }
};
</script>

<style scoped>
.truncate {
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.clip {
  white-space: nowrap;
  overflow: hidden;
  text-overflow: clip;
}
</style>
