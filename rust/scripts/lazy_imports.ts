#!/usr/bin/env -S deno run --allow-read --allow-write

function fmtImport(crate: string, expression: string): string {
  const isBareVersionExpression = !(expression.includes("{") &&
    expression.includes("}"));

  switch (isBareVersionExpression) {
    case true: {
      return `${crate} = { version = ${expression}, optional = true }`;
    }
    case false: {
      expression = expression.replace("{", "").replace("}", "").trim();
      return `${crate} = { ${expression}, optional = true }`;
    }
  }
}

async function main() {
  const imports = await Deno.readTextFile("imports");
  const importsLines = imports.split("\n");

  const features = importsLines.reduce((acc, line) => {
    if (line === "") return acc;
    const eq = line.indexOf("=");
    const [crate, expression]: [string, string] = [
      line.slice(0, eq - 1),
      line.slice(eq + 1),
    ] as [
      string,
      string,
    ];

    return [...acc, `${fmtImport(crate, expression)}`];
  }, [] as string[]);

  console.log(features.join("\n"));
}

await main();
