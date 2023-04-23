export interface Query {
  text: string;
  fallback: string;
}

export interface LookupOptions {
  split?: number;
  follow?: boolean;
}
