import * as fs from "node:fs";
import type { Monkey } from "./lib";

/**
 * Main function
 *
 * @export
 */
export function main(): void {

  if (process.argv[2] == undefined) {
    console.error("path pls");
    process.exit(1);
  }

  const path: string = process.argv[2];

  const monkeys: Monkey[] = parseInput(path);
  console.log("monkeys:");
  monkeys.forEach((monkey) => { console.log(monkey); });

  for (let i = 1; i <= 1000; i++){
    for (const monkey of monkeys) {
      const itemsCopy = monkey.items.slice(); // Create a copy of the items array
      for (let item of itemsCopy) {
        monkey.monkeyBussiness++;
        item = BigInt(monkey.operation(item));
        monkey.items.splice(monkey.items.indexOf(item), 1);
        monkeys[monkey.throwTo(item)].items.push(item);
      }
    }

    console.log(`After round ${i}, ${monkeys.length} monkeys are holding items with the following worry levels:`);
    monkeys.forEach(m => console.log(`Monkey ${m.id}: ${m.items}`));
    console.log("wtf?");
  }

  console.log("The monkey bussiness levels:");
  monkeys.forEach(m => console.log(`Monkey ${m.id}: ${m.monkeyBussiness}`));
}

/**
 * Description placeholder
 *
 * @param {string} path The path to the input file to parse.
 * @returns {Monkey[]} An array of Monkey objects, constructed from the file.
 */
function parseInput(path: string): Monkey[] {
  const file = fs.readFileSync(path, "utf-8");
  const contents: string[] = file.split("\n\n");

  return contents.map((monkeyLines) => {

    // get the relevant lines for each monkey
    const lines: string[] = monkeyLines.split("\n");

    // construct the monkey
    return {
      // "Monkey 0: " =>
      id: parseInt(lines[0].trim().slice("Monkey ".length, "Monkey x".length), 10),

      // "  Starting items: 79, 98" =>
      items: lines[1]
        .trim()
        .slice("Starting items: ".length)
        .split(",")
        .map(s => BigInt(parseInt(s.trim(), 10))),

      // "  Operation: new = old * 19" =>
      operation: (item: bigint) => eval(lines[2].replace("Operation: new = ", "").replaceAll("old", item + "")),

      /* "  Test: divisible by 23
              If true: throw to monkey 2
              If false: throw to monkey 3"  => 
      */
      throwTo: (thrownItem: bigint): number => thrownItem % BigInt(parseInt(lines[3].replace("  Test: divisible by ", "").trim())) == BigInt(0) ?
        parseInt(lines[4].replace("    If true: throw to monkey ", "").trim())
        : parseInt(lines[5].replace("    If false: throw to monkey ", "").trim()),

      monkeyBussiness: 0
    };
  });
}


main();
