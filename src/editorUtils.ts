import { javascript } from "@codemirror/lang-javascript";
import { python } from "@codemirror/lang-python";
import { rust } from "@codemirror/lang-rust";
import { java } from "@codemirror/lang-java";

import {markdown} from "@codemirror/lang-markdown"


export function normalizePath(path: string): string {
    return path.replace(/\\/g, "/");
}

export function debounce(func: (...args: any[]) => void, wait: number) {
    let timeout: ReturnType<typeof setTimeout>;
    return (...args: any[]) => {
        clearTimeout(timeout);
        timeout = setTimeout(() => func(...args), wait);
    };
}


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
  md: markdown
};
