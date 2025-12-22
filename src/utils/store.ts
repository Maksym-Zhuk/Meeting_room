import { Stronghold } from "@tauri-apps/plugin-stronghold";
import { appDataDir } from "@tauri-apps/api/path";
import { invoke } from "@tauri-apps/api/core";

let cachedStronghold: { stronghold: Stronghold; client: any } | null = null;

export async function initStronghold() {
  if (cachedStronghold) {
    console.log("Using cached Stronghold");
    return cachedStronghold;
  }

  try {
    const vaultPath = `${await appDataDir()}/vault.hold`;

    const password: string = await invoke("get_or_create_stronghold_key");

    const stronghold = await Stronghold.load(vaultPath, password);

    let client;
    try {
      client = await stronghold.loadClient("auth");
    } catch {
      client = await stronghold.createClient("auth");
    }

    cachedStronghold = { stronghold, client };

    return cachedStronghold;
  } catch (error: any) {
    console.error("❌ Stronghold init failed:", error);

    if (error.toString().includes("BadFileKey")) {
      await invoke("reset_stronghold");

      return initStronghold();
    }

    throw error;
  }
}

export async function insertRecord(key: string, value: string) {
  const { stronghold, client } = await initStronghold();
  const data = Array.from(new TextEncoder().encode(value));
  await client.getStore().insert(key, data);
  await stronghold.save();
}

export async function getRecord(key: string): Promise<string | null> {
  try {
    const { client } = await initStronghold();
    const data = await client.getStore().get(key);
    if (!data) return null;
    return new TextDecoder().decode(new Uint8Array(data));
  } catch (error) {
    console.error("Failed to get record:", error);
    return null;
  }
}

export async function deleteRecord(key: string) {
  const { client } = await initStronghold();
  await client.getStore().remove(key);
  await client.getStore().save();
}

export async function resetStronghold() {
  await invoke("reset_stronghold");
  cachedStronghold = null;
  console.log("✅ Stronghold reset");
}
