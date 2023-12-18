const { colors } = require("./src/config/config.json");

module.exports = {
  mode: "all",
  content: [
    "./src/**/*.rs",
    "./index.html",
    "./src/**/*.html",
    "./src/**/*.css",
  ],
  darkMode: "class", // or 'media' or 'class'
  theme: {
    colors: {
      transparent: "transparent",
      current: "currentColor",
      ...colors,
    },
    extend: {
      keyframes: {
        twinkle: {
          '0%, 100%': {
            opacity: '1', // Fully opaque
            color: 'rgba(0, 128, 128, 1)',
          },
          '50%': {
            opacity: '0.5', // Semi-transparent
            color: 'rgba(0, 128, 128, 0.5)',
          },
        }
      },

      animation: {
        twinkle: 'twinkle 2s ease-in-out infinite',
      }
    },
    variants: {},
    plugins: [],
  },
};
