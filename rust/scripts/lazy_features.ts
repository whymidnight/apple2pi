#!/usr/bin/env -S deno run --allow-read --allow-write

async function main() {
  const imports = await Deno.readTextFile("imports");
  const importsLines = imports.split("\n");

  const features = importsLines.reduce((acc, line) => {
    const [crate, _]: [string, string] = line.split(" = ") as [
      string,
      string,
    ];

    return [...acc, `"${crate}"`];
  }, [] as string[]);

  console.log(features.join(", "));
}

await main();
