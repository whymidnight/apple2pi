#!/usr/bin/env -S deno run --allow-read --allow-write

const vdevEntry = (c: string) =>
  `("${c}".to_string(), VdevKey::None(Key::Layout('${c}')))`;

async function main() {
  const alphabet = Array.from(Array(26)).map((e, i) => i + 65).map((x) =>
    String.fromCharCode(x).toLowerCase()
  );
  const vdevMap = alphabet.reduce((acc, c) => {
    return [
      ...acc,
      vdevEntry(c),
      vdevEntry(c.toUpperCase()),
    ];
  }, [] as string[]);
  console.log(vdevMap);

  const encoder = new TextEncoder();
  const data = encoder.encode(vdevMap.join(",\n"));
  await Deno.writeFile("vdev_map", data);
}

await main();
