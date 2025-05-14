module.exports = {
    mode: "jit",
    content: [
      "./src/**/*.rs",
      "./index.html",
      "./dist/index.html"
    ],
    darkMode: "media", // 'media' or 'class'
    theme: {
      extend: {},
    },
    variants: {
      extend: {},
    },
    plugins: [],
  };