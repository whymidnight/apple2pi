#!/usr/bin/env -S deno run --allow-read --allow-write

function hidFmt(
  // CTRL-KEY_S
  // KEY_S
  // SHIFT+KEY_S
  sequence: string,
): string {
  const fmt = (char: string) => `Keyboard::${char}`;

  const parts = sequence.split("+");
  const buffer = [];
  for (const [idx, part] of parts.entries()) {
    if (part === "") continue;
    switch (part) {
      case "CTRL": {
        buffer.push("LeftControl");
        break;
      }
      case "SHIFT": {
        buffer.push("LeftShift");
        break;
      }
      default: {
        if (part.includes("KEY_")) {
          const k = part.split("KEY_")[1];
          if (!isNaN(parseInt(k))) {
            buffer.push(`Keyboard${k}`);
          } else {
            buffer.push(
              k.replace(
                /\w+/g,
                (w) => w[0].toUpperCase() + w.slice(1).toLowerCase(),
              ),
            );
          }
        }
      }
    }
  }

  return buffer.map((buf) => fmt(buf)).join(", ");
}

async function main() {
  const kbmapText = await Deno.readTextFile("../a2pi_keymaps/kbmap.json");
  const kbmap = JSON.parse(kbmapText);

  let bufferLeft = "";
  let bufferRight = "";
  bufferLeft += "";
  for (const layer of Object.keys(kbmap)) {
    bufferLeft += "(\n";
    bufferLeft += `  "${layer}", [\n`;

    Object.keys(kbmap[layer]).map(
      (scanCode: string) => {
        const { key, action }: { key: string; action: string } =
          kbmap[layer][scanCode];
        console.log(scanCode, key);
        bufferLeft += "    (\n";
        const keyUp = scanCode;
        const keyDown = `0x${
          (0x80 | Number(keyUp)).toString(16).toUpperCase()
        }`;
        console.log("keyup ::", keyUp, "keydown ::", keyDown);
        bufferLeft +=
          `      "${keyUp.toLowerCase()}", (${keyDown.toLowerCase()}, ${keyUp.toLowerCase()}, vec![${
            hidFmt(action)
          }]) // ${key} :: ${action}\n    ),\n`;
      },
    );
  }
  bufferRight += "]),";

  const buffer = bufferLeft + bufferRight;

  const encoder = new TextEncoder();
  const data = encoder.encode(buffer);
  await Deno.writeFile("pico_map", data);
}

await main();
