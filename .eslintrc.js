module.exports = {
  parser: "babel-eslint",
  extends: "eslint:recommended",
  env: {
    es6: true,
    browser: true,
  },
  plugins: ["svelte3"],
  overrides: [
    {
      files: ["*.svelte"],
      processor: "svelte3/svelte3",
    },
  ],
  rules: {
    indent: ["error", 2],
  },
};
