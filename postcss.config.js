/** @type {import('postcss').AcceptedPlugin} */
export default {
  content: ["./index.html", "./src/**/*.{vue,js,ts,jsx,tsx}"],
  plugins: {
    "@tailwindcss/postcss": {},
  },
};
