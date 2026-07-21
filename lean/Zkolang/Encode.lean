/-
 zKølang by NØNOS
 AGPL-3.0-or-later
-/

/-!
The encoding gadgets pack several small values into one field element at fixed widths. Their
soundness is that the packing is lossless while each part stays inside its width: the packed
value lies in the combined range, and it determines the parts uniquely, so a packed field
element can be read back to exactly one tuple. These are the positional-numeral facts, proven
over the integers, so they carry to the field the compiler targets while the parts are in
range. Reading a part back out is a witnessed decomposition; see `Bits`.
-/

namespace Zkolang.Encode

def pack2_byte (lo hi : Int) : Int := lo + 256 * hi
def pack2_word (lo hi : Int) : Int := lo + 65536 * hi
def pack3_byte (a b c : Int) : Int := a + 256 * b + 65536 * c
def pack4_byte (a b c d : Int) : Int := a + 256 * b + 65536 * c + 16777216 * d

/-- Packing two bytes yields a sixteen-bit value: the packing does not overflow its width. -/
theorem pack2_byte_range (lo hi : Int)
    (hlo : 0 ≤ lo) (hlo' : lo < 256) (hhi : 0 ≤ hi) (hhi' : hi < 256) :
    0 ≤ pack2_byte lo hi ∧ pack2_byte lo hi < 65536 := by
  unfold pack2_byte; omega

/-- Two bytes are recovered uniquely from their packing: the encoding is injective in range. -/
theorem pack2_byte_injective (lo hi lo' hi' : Int)
    (hlo : 0 ≤ lo) (hlo' : lo < 256) (_hhi : 0 ≤ hi) (_hhi' : hi < 256)
    (klo : 0 ≤ lo') (klo' : lo' < 256) (_khi : 0 ≤ hi') (_khi' : hi' < 256)
    (heq : pack2_byte lo hi = pack2_byte lo' hi') :
    lo = lo' ∧ hi = hi' := by
  unfold pack2_byte at heq; omega

/-- Two sixteen-bit words are recovered uniquely from their packing. -/
theorem pack2_word_injective (lo hi lo' hi' : Int)
    (hlo : 0 ≤ lo) (hlo' : lo < 65536) (_hhi : 0 ≤ hi) (_hhi' : hi < 65536)
    (klo : 0 ≤ lo') (klo' : lo' < 65536) (_khi : 0 ≤ hi') (_khi' : hi' < 65536)
    (heq : pack2_word lo hi = pack2_word lo' hi') :
    lo = lo' ∧ hi = hi' := by
  unfold pack2_word at heq; omega

/-- Three bytes are recovered uniquely from their packing. -/
theorem pack3_byte_injective (a b c a' b' c' : Int)
    (ha : 0 ≤ a) (ha' : a < 256) (hb : 0 ≤ b) (hb' : b < 256) (_hc : 0 ≤ c) (_hc' : c < 256)
    (ka : 0 ≤ a') (ka' : a' < 256) (kb : 0 ≤ b') (kb' : b' < 256) (_kc : 0 ≤ c') (_kc' : c' < 256)
    (heq : pack3_byte a b c = pack3_byte a' b' c') :
    a = a' ∧ b = b' ∧ c = c' := by
  unfold pack3_byte at heq; omega

/-- Four bytes are recovered uniquely from their packing. -/
theorem pack4_byte_injective (a b c d a' b' c' d' : Int)
    (ha : 0 ≤ a) (ha' : a < 256) (hb : 0 ≤ b) (hb' : b < 256)
    (hc : 0 ≤ c) (hc' : c < 256) (_hd : 0 ≤ d) (_hd' : d < 256)
    (ka : 0 ≤ a') (ka' : a' < 256) (kb : 0 ≤ b') (kb' : b' < 256)
    (kc : 0 ≤ c') (kc' : c' < 256) (_kd : 0 ≤ d') (_kd' : d' < 256)
    (heq : pack4_byte a b c d = pack4_byte a' b' c' d') :
    a = a' ∧ b = b' ∧ c = c' ∧ d = d' := by
  unfold pack4_byte at heq; omega

end Zkolang.Encode
