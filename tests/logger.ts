import pino from "pino";
import path from "node:path";

const logFileDestination = pino.destination({
  dest: path.join(".", "./logs/app.log"),
  sync: false,
  mkdir: true,
});

export const logger = pino(
  {
    level: "debug",
  },
  pino.multistream([{ stream: logFileDestination, level: "debug" }])
);
