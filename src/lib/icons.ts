export const extensionToClass = {
  c: `<i class="devicon-c-plain"></i>`,
  cpp: `<i class="devicon-cplusplus-plain"></i>`,
  java: `<i class="devicon-java-plain"></i>`,
  py: `<i class="devicon-python-plain"></i>`,
  js: `<i class="devicon-javascript-plain"></i>`,
  jsx: `<i class="devicon-react-plain"></i>`,
  ts: `<i class="devicon-typescript-plain"></i>`,
  tsx: `<i class="devicon-react-plain"></i>`,
  rs: `<i class="devicon-rust-plain"></i>`,
  html: `<i class="devicon-html5-plain"></i>`,
  json: `<i class="devicon-json-plain"></i>`,
  css: `<i class="devicon-css3-plain"></i>`,
  poetry: `<i class="devicon-poetry-plain"></i>`,
  md: `<i class="devicon-markdown-plain"></i>`,
  svelte: `<i class="devicon-svelte-plain"></i>`,
  go: `<i class="devicon-go-plain"></i>`,
  gitignore: `<i class="devicon-git-plain"></i>`,
  gitattributes: `<i class="devicon-git-plain"></i>`,
};

export function fileToIcon(name: string) {
  return (
    extensionToClass[name.split(".").at(-1) as keyof typeof extensionToClass] ||
    `<i class="bi bi-file-earmark-code-fill"></i>`
  );
}
