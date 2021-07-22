const m = {};

import("./pkg").then((module) => {
  m.evaluate = module.evaluate;
});

// window.exec = function exec() {
//   const el = document.getElementById("code");
//   const code = el.value;
//   const result = m.evaluate(code);
//   console.log("result: ", result);
// };

// window.changed = function changed(e) {
//   console.log("changed", e);
// };

// let a = 0.1;
// let b = 0.2;
// a + b

new Vue({
  el: "#app",
  data() {
    return {
      code: "",
    };
  },
  methods: {
    changed(e) {
      if (e.metaKey && e.code === "Enter") {
        this.exec();
      }
    },
    exec() {
      console.clear();
      try {
        const result = m.evaluate(this.code);
        console.log(result);
      } catch (err) {
        console.log(err);
      }
    },
  },
});
