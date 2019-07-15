
<template>
  <div class="overlay">
    <h1>{{ players[0].name }} {{ players[0].score }}</h1>
    <h1>{{ players[1].name }} {{ players[1].score }}</h1>
  </div>
</template>

<script>
import axios from "axios";

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
      // TODO this needs to execute every few seconds
    [0, 1].forEach(element => {
      var endpoint = `/api/${this.appKey}/player/${element}`;
      console.log("fetching from", endpoint);
      axios
        .get(endpoint)
        .then(response => {
          console.log(response);
          this.players[element].name = response.data.name;
          this.players[element].score = response.data.score;
        })
        .catch(e => {
          console.log(e);
        });
    });
  }
};
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
h3 {
  margin: 40px 0 0;
}
ul {
  list-style-type: none;
  padding: 0;
}
li {
  display: inline-block;
  margin: 0 10px;
}
a {
  color: #42b983;
}
</style>
