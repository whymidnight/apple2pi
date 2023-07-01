#!/usr/bin/env -S deno run --allow-read --allow-write

type KeyMap = {
  // `keyDownScanCode` is the emitted scan code from the Apple II.
  [keyDownScanCode: string]: {
    // `key` annotates the rendered character.
    key: string;
    // `action` is the rendered input sequence.
    action: string;
  };
};

async function main() {
  // do something
  const kbmap = await Deno.readTextFile("../../src/kbmap.h");
  const kbmapLines = kbmap.split("\n").slice(5, 133);

  const kbmapped = kbmapLines.reduce((acc, line) => {
    // line = line.split("// ")[1];
    const [cmd, meta]: [string, string] = line.split("//") as [
      string,
      string,
    ];
    let [key, code]: [string, string] = meta.split("code") as [
      string,
      string,
    ];
    key = key.trim(), code = code.trim();

    return {
      ...acc,
      [`0x${code}`]: {
        key,
        action: (() => {
          const scanCode = [];
          for (const k of cmd.split(",")[0].split("|")) {
            const kTrimmed = k.trim();
            switch (kTrimmed) {
              case "MOD_CTRL": {
                scanCode.push("CTRL");
                break;
              }
              case "MOD_SHIFT": {
                scanCode.push("SHIFT");
                break;
              }
              default: {
                scanCode.push(kTrimmed);
                break;
              }
            }
          }
          return scanCode.join("+");
        })(),
      },
    };
  }, {} as KeyMap);

  const encoder = new TextEncoder();
  const data = encoder.encode(JSON.stringify(kbmapped, null, 2));
  await Deno.writeFile("../a2pi_keymaps/kbmap.json", data);
}

await main();
