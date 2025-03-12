import { type Config } from "tailwindcss";

export default {
  content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
  darkMode: "class",
  plugins: [],
} satisfies Config;
