#!/usr/bin/env python3
"""Force the UF2 family ID on every block to RP2350-ARM-S.

Why this exists: our flashing tool `elf2uf2-rs` (2.2.0, the version currently in
devbox) auto-detects the wrong chip and tags our RP2350 firmware with the RP2040
family ID. The flash *payload* is correct — only the 4-byte family-ID field in each
512-byte UF2 block header is wrong, and the RP2350 bootrom rejects a file whose
family doesn't match. This rewrites just that field, in place. Idempotent: running
it again on an already-correct file is a no-op.

The clean long-term fix is `picotool` (which sets the family correctly), but its
nixpkgs build is broken at the moment — see firmware/README or the justfile.

Usage: python3 tools/uf2_set_family.py <file.uf2>
"""
import struct
import sys

RP2040 = 0xE48BFF56
RP2350_ARM_S = 0xE48BFF59
UF2_MAGIC0 = 0x0A324655
BLOCK = 512

path = sys.argv[1]
data = bytearray(open(path, "rb").read())
assert len(data) % BLOCK == 0, f"not a whole number of 512-byte blocks: {len(data)}"

patched = 0
for off in range(0, len(data), BLOCK):
    # UF2 block header: magicStart0 @0, magicStart1 @4, flags @8, ..., familyID @28.
    magic0 = struct.unpack_from("<I", data, off)[0]
    assert magic0 == UF2_MAGIC0, f"bad magic at block {off // BLOCK}"
    (fam,) = struct.unpack_from("<I", data, off + 28)
    if fam == RP2040:
        struct.pack_into("<I", data, off + 28, RP2350_ARM_S)
        patched += 1
    elif fam != RP2350_ARM_S:
        sys.exit(f"unexpected family 0x{fam:08x} at block {off // BLOCK}")

open(path, "wb").write(data)
print(f"patched {patched}/{len(data) // BLOCK} blocks -> RP2350-ARM-S (0x{RP2350_ARM_S:08x})")
