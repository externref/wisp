import { javascript } from "@codemirror/lang-javascript";
import { python } from "@codemirror/lang-python";
import { rust } from "@codemirror/lang-rust";
import { java } from "@codemirror/lang-java";
import { markdown } from "@codemirror/lang-markdown";
import { css } from "@codemirror/lang-css";
import { html } from "@codemirror/lang-html";
import { sql } from "@codemirror/lang-sql";
import { yaml } from "@codemirror/lang-yaml";
import { xml } from "@codemirror/lang-xml";
import { go } from "@codemirror/lang-go";

import * as themes from "thememirror";
import type { ThemeSchema } from "../lib/interfaces";

export const themeNames = Object.keys(themes)
  // @ts-ignore
  .map((key) => themes[key as keyof typeof themes].name || key)
  .filter((val) => {
    return val != "createTheme";
  });

export const themeSchemes: { [x: string]: ThemeSchema } = {
  amy: {
    background: "#121212",
    foreground: "#EAEAEA",
    primary: "#FF5733",
    secondary: "#C70039",
    accent: "#900C3F",
  },
  ayuLight: {
    background: "#1E1E1E",
    foreground: "#B3B9C5",
    primary: "#F29718",
    secondary: "#39ADB5",
    accent: "#FFD580",
  },
  barf: {
    background: "#333333",
    foreground: "#F5F5DC",
    primary: "#98FB98",
    secondary: "#FFA07A",
    accent: "#FF6347",
  },
  bespin: {
    background: "#282828",
    foreground: "#D3D0C8",
    primary: "#AB82FF",
    secondary: "#8FBC8F",
    accent: "#EECFA1",
  },
  birdsOfParadise: {
    background: "#1C1C1C",
    foreground: "#F5DEB3",
    primary: "#FF7F50",
    secondary: "#7CFC00",
    accent: "#FFB6C1",
  },
  boysAndGirls: {
    background: "#232323",
    foreground: "#F0E68C",
    primary: "#FF69B4",
    secondary: "#87CEEB",
    accent: "#FFFACD",
  },
  clouds: {
    background: "#2C2C2C",
    foreground: "#F5F5F5",
    primary: "#00CED1",
    secondary: "#FF4500",
    accent: "#FA8072",
  },
  cobalt: {
    background: "#002240",
    foreground: "#A9A9A9",
    primary: "#FFA500",
    secondary: "#32CD32",
    accent: "#4682B4",
  },
  coolGlow: {
    background: "#202040",
    foreground: "#E0E0E0",
    primary: "#FF1493",
    secondary: "#00FFFF",
    accent: "#7CFC00",
  },
  createTheme: {
    background: "#191919",
    foreground: "#DCDCDC",
    primary: "#1E90FF",
    secondary: "#B22222",
    accent: "#FF8C00",
  },
  dracula: {
    background: "#282A36",
    foreground: "#F8F8F2",
    primary: "#6272A4",
    secondary: "#FF79C6",
    accent: "#50FA7B",
  },
  espresso: {
    background: "#1C1B1A",
    foreground: "#F0F0E0",
    primary: "#D2B48C",
    secondary: "#FFD700",
    accent: "#FFA07A",
  },
  noctisLilac: {
    background: "#232136",
    foreground: "#D8BFD8",
    primary: "#BA55D3",
    secondary: "#EE82EE",
    accent: "#98FB98",
  },
  rosePineDawn: {
    background: "#2F2F4F",
    foreground: "#E6E6FA",
    primary: "#FF69B4",
    secondary: "#4682B4",
    accent: "#DA70D6",
  },
  smoothy: {
    background: "#2B2B2B",
    foreground: "#FFFFFF",
    primary: "#FF6347",
    secondary: "#8FBC8F",
    accent: "#FFDEAD",
  },
  solarizedLight: {
    background: "#073642",
    foreground: "#93A1A1",
    primary: "#268BD2",
    secondary: "#2AA198",
    accent: "#B58900",
  },
  tomorrow: {
    background: "#282C34",
    foreground: "#ABB2BF",
    primary: "#61AFEF",
    secondary: "#C678DD",
    accent: "#98C379",
  },
};

export const extensionToLanguageConfig = {
  js: javascript,
  ts: () => javascript({ typescript: true }),
  jsx: () => javascript({ typescript: true, jsx: true }),
  tsx: () => javascript({ typescript: true, jsx: true }),
  py: python,
  rs: rust,
  java: java,
  md: markdown,
  css: css,
  html: html,
  sql: sql,
  yaml: yaml,
  xml: xml,
  go: go,
};
