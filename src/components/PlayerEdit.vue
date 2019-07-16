<template>
  <div class="player-edit">
    <h1>{{ name }} {{ score }}</h1>
    <b-form @reset="onReset" @submit="onSubmit">
      <b-form-select id="name" v-model="name" @change="onNameChanged" :options="possibleNames" />
      <b-form-input id="score" v-model.number="score" @change="onScoreChanged" type="number" />
      <b-button type="submit" variant="primary">Submit</b-button>
      <b-button type="reset" variant="danger">Clear</b-button>
    </b-form>
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
    initialScore: Number,
    possibleNames: Array
  },
  created() {
    var endpoint = `/api/${this.appKey}/player/${this.id}`;
    axios
      .get(endpoint)
      .then(response => {
        this.name = response.data.name;
        this.score = response.data.score;
      })
      .catch();
  },
  methods: {
    onScoreChanged() {
      this.onSubmit();
    },
    onNameChanged() {
      this.score = 0;
      this.onSubmit();
    },
    onSubmit() {
      // Todo this could be a watcher + debounce
      var endpoint = `/api/${this.appKey}/player/${this.id}/update`;
      axios
        .post(endpoint, {
          name: this.name,
          score: this.score
        })
        .catch();
    },
    onReset() {
      this.name = "";
      this.score = 0;
      this.onChanged();
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
