<template>
  <div class="player-edit">
    <h1>{{ id }} {{ name }} {{ score }}</h1>
    <input id="name" v-model="name" @change="onChange" />
    <input id="score" v-model.number="score" @change="onChange" />
  </div>
</template>

<script>
import axios from "axios";

export default {
  name: "PlayerEdit",
  data: function() {
    return {
      name: this.initialName,
      score: this.initialScore
    };
  },
  props: {
    appKey: String,
    id: Number,
    initialName: String,
    initialScore: Number
  },
  methods: {
    onChange(event) {
      console.log(event);
      var endpoint = `/api/${this.appKey}/player/${this.id}/update`;
      axios
        .post(endpoint, {
          name: this.name,
          score: this.score
        })
        .then(response => {
          console.log(response);
        })
        .catch(e => {
          console.log(e);
        });
    }
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
