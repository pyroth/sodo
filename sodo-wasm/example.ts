/**
 * sodo-wasm usage example with Bun
 * Run: bun run example.ts
 */

import init, {
  Difficulty,
  generateSudoku,
  solveGrid,
  validateSolution,
  validateGrid,
  isSolvable,
  getHint,
  formatGrid,
  createEmptyGrid,
  cloneGrid,
  gridToJson,
  jsonToGrid,
  parseGrid,
  stringifyGrid,
  // String API
  generate,
  solve,
  validate,
  hint,
  format,
} from "./pkg/sodo_wasm.js";

// Initialize WASM module
await init();

console.log("=== sodo-wasm Example ===\n");

// 1. Generate a puzzle with Grid API
console.log("1. Generating puzzle (Medium difficulty)...");
const { puzzle, solution } = generateSudoku(Difficulty.Medium);
console.log("Puzzle:");
console.log(formatGrid(puzzle));
console.log("Solution:");
console.log(formatGrid(solution));

// 2. Validate the solution
console.log("2. Validating solution...");
const isValid = validateSolution(puzzle, solution);
console.log(`Solution valid: ${isValid}\n`);

// 3. Get a hint
console.log("3. Getting hint for puzzle...");
const hintResult = getHint(puzzle);
if (hintResult) {
  console.log(`Hint: Place ${hintResult.value} at row ${hintResult.row}, col ${hintResult.col}\n`);
} else {
  console.log("No hint available\n");
}

// 4. Check if puzzle is solvable
console.log("4. Checking if puzzle is solvable...");
console.log(`Is solvable: ${isSolvable(puzzle)}\n`);

// 5. Grid utilities
console.log("5. Grid utilities demo...");
const emptyGrid = createEmptyGrid();
console.log(`Empty grid created: ${emptyGrid[0][0] === 0 ? "✓" : "✗"}`);

const cloned = cloneGrid(puzzle);
console.log(`Grid cloned: ${cloned[0][0] === puzzle[0][0] ? "✓" : "✗"}`);

// 6. JSON conversion
console.log("\n6. JSON conversion...");
const json = gridToJson(puzzle);
console.log(`Grid to JSON (first 50 chars): ${json.slice(0, 50)}...`);

const fromJson = jsonToGrid(json);
console.log(`JSON to Grid: ${fromJson ? "✓" : "✗"}`);

// 7. String conversion
console.log("\n7. String conversion...");
const compactStr = stringifyGrid(puzzle);
console.log(`Compact string: ${compactStr}`);

const fromStr = parseGrid(compactStr);
console.log(`Parse string: ${fromStr ? "✓" : "✗"}`);

// 8. String API (legacy)
console.log("\n8. String API demo...");
const puzzleStr = generate("hard");
console.log(`Generated (hard): ${puzzleStr}`);

const solutionStr = solve(puzzleStr);
console.log(`Solved: ${solutionStr}`);

console.log(`Valid: ${validate(puzzleStr)}`);

const hintStr = hint(puzzleStr);
if (hintStr) {
  console.log(`Hint: row=${hintStr.row}, col=${hintStr.col}, value=${hintStr.value}`);
}

console.log("\n=== Done ===");
