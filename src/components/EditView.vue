<template>
  <div id="editView" class="container">
    <div class="row">
      <div class="col-sm">
        <PlayerEdit
          :id="0"
          :app-key="appKey"
          :initial-name="defaultName"
          :initial-score="defaultScore"
          :possible-names="allNames"
          ref="leftPlayer"
        />
      </div>
      <div class="col-sm">
        <PlayerEdit
          :id="1"
          :app-key="appKey"
          :initial-name="defaultName"
          :initial-score="defaultScore"
          :possible-names="allNames"
          ref="rightPlayer"
        />
      </div>
    </div>
    <div class="row">
      <div class="col-sm">
        <b-button variant="primary" @click="swapPlayers">Swap</b-button>
      </div>
    </div>
  </div>
</template>

<script>
import axios from "axios";
import PlayerEdit from "./PlayerEdit.vue";

export default {
  name: "EditView",
  data: function() {
    return {
      defaultName: "Unknown",
      defaultScore: 0,
      allNames: ["Unknown"]
    };
  },
  components: {
    PlayerEdit
  },
  computed: {
    appKey: function() {
      return this.$route.params.appKey;
    }
  },
  created() {
    // Load names from API
    var endpoint = `/api/${this.appKey}/names`;
    axios
      .get(endpoint)
      .then(response => {
        this.allNames = Array.from(response.data)
          .map(f => f.name)
          .sort();
        this.allNames.unshift("");
      })
      .catch();
  },
  methods: {
    swapPlayers() {
      var endpoint = `/api/${this.appKey}/swap`;
      axios
        .post(endpoint, {
          first_player: 0,
          second_player: 1
        })
        .then(() => {
          this.$refs.leftPlayer.syncData();
          this.$refs.rightPlayer.syncData();
        })
        .catch();
    }
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
