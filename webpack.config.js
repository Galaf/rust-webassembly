const path = require("path");

module.exports = {
  entry: "./static/js/index.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "index.js",
  },
  mode: "development",
};
