import { createGlobalState, useStorage } from "@vueuse/core";

export const useGlobalState = createGlobalState(() =>
  useStorage("state", {
    apiKey: "",
    theme: "system",
  }),
);
