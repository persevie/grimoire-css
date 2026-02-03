// Node 18+ provides global fetch. This is only to satisfy TS when configured environments vary.

type MinimalFetchResponse = {
  ok: boolean;
  status: number;
  statusText?: string;
  json(): Promise<unknown>;
  text(): Promise<string>;
  arrayBuffer(): Promise<ArrayBuffer>;
};

type MinimalFetchInit = {
  method?: string;
  headers?: Record<string, string>;
  body?: string;
};

type MinimalFetch = (input: string, init?: MinimalFetchInit) => Promise<MinimalFetchResponse>;

declare const fetch: MinimalFetch;

export {};
