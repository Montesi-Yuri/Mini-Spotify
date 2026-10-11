import { invoke } from "@tauri-apps/api/core";

export const player = {
  playPause: () => invoke("play_pause"),
  next: () => invoke("next"),
  prev: () => invoke("prev"),
  seek: (positionMs: number) => invoke("seek", { positionMs }),
  setVolume: (percent: number) =>
    invoke("set_volume", { volume: Math.round((percent / 100) * 65535) }),
}
