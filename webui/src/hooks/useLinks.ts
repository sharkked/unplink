import { ref } from "vue";

const HOST = "http://localhost:3000";

export function useLinks() {
  const token = ref<string>("");
  const lastResult = ref<string>("");

  const shorten = async (url: string) => {
    const shortLink = await fetch(HOST, {
      method: "POST",
      headers: {
        Authorization: token.value,
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ url: url }),
    });
    const data = await shortLink.json();
    lastResult.value = data.url;
  };

  return { token, lastResult, shorten };
}
