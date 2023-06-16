"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || function (mod) {
    if (mod && mod.__esModule) return mod;
    var result = {};
    if (mod != null) for (var k in mod) if (k !== "default" && Object.prototype.hasOwnProperty.call(mod, k)) __createBinding(result, mod, k);
    __setModuleDefault(result, mod);
    return result;
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.main = void 0;
const fs = __importStar(require("node:fs"));
/**
 * Main function
 *
 * @export
 */
function main() {
    if (process.argv[2] == undefined) {
        console.error("path pls");
        process.exit(1);
    }
    const path = process.argv[2];
    const monkeys = parseInput(path);
    console.log("monkeys:");
    monkeys.forEach((monkey) => { console.log(monkey); });
    for (let i = 1; i <= 20; i++) {
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
exports.main = main;
/**
 * Description placeholder
 *
 * @param {string} path The path to the input file to parse.
 * @returns {Monkey[]} An array of Monkey objects, constructed from the file.
 */
function parseInput(path) {
    const file = fs.readFileSync(path, "utf-8");
    const contents = file.split("\n\n");
    return contents.map((monkeyLines) => {
        // get the relevant lines for each monkey
        const lines = monkeyLines.split("\n");
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
            operation: (item) => eval(lines[2].replace("Operation: new = ", "").replaceAll("old", item + "")),
            /* "  Test: divisible by 23
                    If true: throw to monkey 2
                    If false: throw to monkey 3"  =>
            */
            throwTo: (thrownItem) => thrownItem % BigInt(parseInt(lines[3].replace("  Test: divisible by ", "").trim())) == BigInt(0) ?
                parseInt(lines[4].replace("    If true: throw to monkey ", "").trim())
                : parseInt(lines[5].replace("    If false: throw to monkey ", "").trim()),
            monkeyBussiness: 0
        };
    });
}
main();
//# sourceMappingURL=index.js.map