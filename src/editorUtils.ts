import { javascript } from "@codemirror/lang-javascript";
import { python } from "@codemirror/lang-python";
import { rust } from "@codemirror/lang-rust";
import { java } from "@codemirror/lang-java";
export const extensionToLanguageConfig = {
  js: javascript,
  ts: () => {
    return javascript();
  },
  jsx: javascript,
  tsx: javascript,
  py: python,
  rs: rust,
  java: java,
};
