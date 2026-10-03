/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*
 * node check.mjs <module.wasm> <fixture dir>: the module accepts the fixture's proof, and
 * refuses it with a byte flipped and with another output. Exits 1 if any of that fails.
 */

import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { load, verify } from './zkolang-verify.mjs';

const [wasm, dir] = process.argv.slice(2);
const m = await load(readFileSync(wasm));
const read = (name) => readFileSync(join(dir, name));
const image = read('program.bin');
const statement = read('statement.txt').toString();
const proof = read('proof.bin');
const slots = read('words.txt').toString().trim().split('\n').slice(5).map(BigInt);

const bad = Uint8Array.from(proof);
bad[bad.length >> 1] ^= 1;
const other = [...slots.slice(0, -1), slots[slots.length - 1] + 1n];
const cases = [
  ['the pinned proof', verify(m, { image, statement, proof, slots }), true],
  ['a flipped byte', verify(m, { image, statement, proof: bad, slots }), false],
  ['another output', verify(m, { image, statement, proof, slots: other }), false],
];
let failed = 0;
for (const [what, r, want] of cases) {
  const held = r.ok === want;
  failed += held ? 0 : 1;
  console.log(`${held ? 'ok  ' : 'FAIL'} ${what}: code ${r.code}${r.reason ? `, ${r.reason}` : ''}`);
}
process.exit(failed === 0 ? 0 : 1);
