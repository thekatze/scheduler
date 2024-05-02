/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["./src/**/*.rs"],
    extract: {
      rs: (content) => {
        return content.match(/\.([\w:\/_-]+)/g)?.map((str) => str.substring(1)) ?? [];
      }
    }
  },
  theme: {
    extend: {},
  },
  plugins: [],
}

