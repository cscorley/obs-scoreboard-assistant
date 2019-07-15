
<template>
  <div id="overlay" class="container-fluid">
    <div class="row">
      <div class="col text-trucate text-right truncate">{{ players[0].name }}</div>
      <div class="col-auto text-center">&#9876;</div>
      <div class="col text-trucate text-left truncate">{{ players[1].name }}</div>
    </div>
    <div class="row">
      <div class="col text-right">{{ players[0].score }}</div>
      <div class="col-auto text-center">&#8226;</div>
      <div class="col text-left">{{ players[1].score }}</div>
    </div>
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
#overlay {
  display: block;
  font-size: 30px;
  font-size: 3.5vw;
}

.truncate {
  width: 100%;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
</style>
