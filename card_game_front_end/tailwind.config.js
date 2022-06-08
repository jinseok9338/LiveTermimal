const { colors } = require("./src/config/config.json");

module.exports = {
  mode: "all",
  content: [
    "./src/**/*.rs",
    "./index.html",
    "./src/**/*.html",
    "./src/**/*.css",
  ],
  darkMode: "media", // or 'media' or 'class'
  theme: {
    colors: {
      transparent: "transparent",
      current: "currentColor",
      ...colors,
    },
    variants: {},
    plugins: [],
  },
};
