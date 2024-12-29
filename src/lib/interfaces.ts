export interface Configs {
  theme: string;
  editor_font_size: number;
  buffer_font_size: number;
}

export interface Session {
  directory: string;
  opened_buffers: { [file: string]: string };
}

export interface FileEntry {
  name: string;
  path: string;
  is_directory: boolean;
  children: FileEntry[];
}

export interface ThemeSchema {
  background: string;
  foreground: string;
  primary: string;
  secondary: string;
  accent: string;
}
