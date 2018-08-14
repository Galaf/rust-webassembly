const path = require("path");

module.exports = {
  entry: "./websrc/js/index.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "index.js",
  },
  mode: "development",
};
