import { writable } from "svelte/store";
import { NUM_CHANNELS } from "./const";

export const osciData = writable([]);

export const channelConfig = writable(
  [
    {
      "channelId": 0,
      "enabled": true,
      "thickness": false,
      "offset": 0,
      "sweepSpeed": 5,
      "amplitude": 1
    },
    {
      "channelId": 1,
      "enabled": true,
      "thickness": false,
      "offset": 0,
      "sweepSpeed": 5,
      "amplitude": 1
    },
    {
      "channelId": 2,
      "enabled": true,
      "thickness": false,
      "offset": 0,
      "sweepSpeed": 5,
      "amplitude": 1
    },
    {
      "channelId": 3,
      "enabled": true,
      "thickness": false,
      "offset": 0,
      "sweepSpeed": 5,
      "amplitude": 1
    },
    {
      "channelId": 4,
      "enabled": true,
      "thickness": false,
      "offset": 0,
      "sweepSpeed": 5,
      "amplitude": 1
    },
    {
      "channelId": 5,
      "enabled": true,
      "thickness": false,
      "offset": 0,
      "sweepSpeed": 5,
      "amplitude": 1
    },
    {
      "channelId": 6,
      "enabled": true,
      "thickness": false,
      "offset": 0,
      "sweepSpeed": 5,
      "amplitude": 1
    },
    {
      "channelId": 7,
      "enabled": true,
      "thickness": false,
      "offset": 0,
      "sweepSpeed": 5,
      "amplitude": 1
    },
    {
      "channelId": 8,
      "enabled": true,
      "thickness": false,
      "offset": 0,
      "sweepSpeed": 5,
      "amplitude": 1
    },
    {
      "channelId": 9,
      "enabled": true,
      "thickness": false,
      "offset": 0,
      "sweepSpeed": 5,
      "amplitude": 1
    },
    {
      "channelId": 10,
      "enabled": true,
      "thickness": false,
      "offset": 0,
      "sweepSpeed": 5,
      "amplitude": 1
    },
  ]
);