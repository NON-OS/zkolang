/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*
 * A zKølang format 7 proof verified in a page or in Node by the module nonos_zkolang_wasm
 * builds. `load(bytes)` instantiates it. `verify(m, { image, statement, proof, slots })`
 * takes the image (`program.bin`), the text of `statement.txt`, the proof, and the public
 * input slots then the output slots as BigInts (SPEC section 12.2); the words the program
 * fixes are read from the statement. It returns `{ ok, code, reason }`.
 */

export async function load(bytes) {
  const { instance } = await WebAssembly.instantiate(bytes, {});
  return instance.exports;
}

/* The `name value` lines of statement.txt. */
export function parse(text) {
  const s = {};
  for (const line of text.split('\n').filter((l) => l.trim() !== '')) {
    const at = line.indexOf(' ');
    s[line.slice(0, at)] = line.slice(at + 1).trim();
  }
  return s;
}

function hex(h) {
  if (!/^[0-9a-f]{64}$/.test(h)) throw new Error(`not a 32-byte hash: ${h}`);
  return Uint8Array.from(h.match(/../g), (b) => parseInt(b, 16));
}

/* `bytes` copied into a buffer of the module's; the buffer's address. */
function put(m, bytes) {
  const at = m.zk_alloc(Math.max(bytes.length, 1));
  new Uint8Array(m.memory.buffer, at, bytes.length).set(bytes);
  return [at, Math.max(bytes.length, 1)];
}

export function verify(m, { image, statement, proof, slots }) {
  const s = parse(statement);
  if (s.format !== '7') throw new Error('not a format 7 statement');
  const head = s.head.split(',').map(BigInt);
  const words = BigUint64Array.from([...head, ...slots.map(BigInt)]);
  const st = new Uint8Array(104 + image.length);
  ['program_hash', 'periodic_root', 'params'].forEach((k, i) => st.set(hex(s[k]), 32 * i));
  new DataView(st.buffer).setBigUint64(96, BigInt(s.words), true);
  st.set(image, 104);
  const bufs = [put(m, st), put(m, proof), put(m, new Uint8Array(words.buffer))];
  const [[sp], [pp], [wp]] = bufs;
  const code = m.zk_verify(sp, st.length, pp, proof.length, wp, words.length);
  const n = m.zk_reason(0, 0);
  const at = m.zk_alloc(Math.max(n, 1));
  m.zk_reason(at, n);
  const reason = new TextDecoder().decode(new Uint8Array(m.memory.buffer, at, n));
  [...bufs, [at, Math.max(n, 1)]].forEach(([p, len]) => m.zk_free(p, len));
  return { ok: code === 0, code, reason };
}
