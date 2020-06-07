<template>
  <div id="setNames">
    <b-form @submit="onSubmit" @reset="onReset">
      <b-form-textarea id="textarea" v-model="text" placeholder="Enter one name per line" rows="10"></b-form-textarea>
      <b-button type="submit" variant="primary">Submit</b-button>
      <b-button type="reset" variant="danger">Reset</b-button>
    </b-form>
  </div>
</template>

<script>
import axios from "axios";

export default {
  name: "setNames",
  data: function() {
    return {
      text: "",
      originalNames: [],
      updatedNames: []
    };
  },
  created() {
    // Load names from API
    var endpoint = `/api/${this.appKey}/names`;
    axios.get(endpoint).then(response => {
      this.setNames(response.data);
    });
  },
  computed: {
    appKey: function() {
      return this.$route.params.appKey;
    }
  },
  methods: {
    setNames(responseData) {
      if (responseData !== undefined) {
        var data = Array.from(responseData)
          .map(f => f.name)
          .sort();
        this.originalNames = data;
        this.text = this.originalNames.join("\n");
      }
    },
    onSubmit() {
      this.updatedNames = this.text
        .split("\n")
        .map(element => element.trim())
        .filter(element => element.length > 0);

      this.text = this.updatedNames.join("\n");

      var endpoint = `/api/${this.appKey}/names/update`;
      axios.post(endpoint, this.updatedNames).then(response => {
        this.setNames(response.data);
      });
    },
    onReset() {
      this.text = this.originalNames.join("\n");
    }
  }
};
</script>

<style scoped>
</style>
