## References

- [SwissTable Design](https://abseil.io/about/design/swisstables)
- [absl::flat_hash_map](https://github.com/abseil/abseil-cpp/blob/master/absl/container/internal/raw_hash_set.h)
- [F14 HashMap](https://github.com/facebook/folly/blob/main/folly/container/F14.md)

# Key Concepts

- 1. Control Byte Metadata

- Each slot has a 1-byte control byte:
  - 0x80 (1000_0000) = EMPTY (MSB set)
  - 0x00-0x7F (0xxxxxxx) = FULL with 7-bit hash tag
  - 0xFE = DELETED tombstone
  - 0xFF = SENTINEL (boundary marker)

- 2. SIMD Acceleration

- The code loads 16 control bytes at once using:
  - ARM NEON (neon.rs) on ARM platforms
  - Scalar fallback (scalar.rs) on other platforms

- 3. Hash Splitting

  h1 = hash >> 7            // Upper 57 bits → slot index
  h2 = (hash >> 57) & 0x7F  // Lower 7 bits → control tag

- 4. Probe Sequence

  - 1. Load 16 control bytes via SIMD
  - 2. Check for matching tags (using BitMask)
  - 3. Check for empty/deleted slots
  - 4. Advance to next group if needed
  - 5. Growth Policy

  - Grows when load factor exceeds 7/8 (87.5%)
  - Rehashes when tombstones ≥ half of live items
  - Capacity always power-of-two for fast modulo

# 주요 개념

- 1. 제어 바이트 메타데이터

- 각 슬롯에는 1바이트 제어 바이트가 있습니다:
  - 0x80 (1000_0000) = 비어 있음 (MSB 세트)
  - 0x00-0x7F (0xxxxxxx) = FULL with 7-bit hash tag
  - 0xFE = 삭제된 묘비
  - 0xFF = SENTINE (boundary 마커)

- 2. SIMD 가속

- 코드는 다음을 사용하여 한 번에 16개의 제어 바이트를 로드합니다:
  - ARM 플랫폼의 ARM 네온(neon.rs )
  - 다른 플랫폼의 스칼라 폴백(scalar.rs )

- 3. 해시 분할

  h1 = 해시 >> 7 // 상위 57비트 → 슬롯 인덱스
  h2 = (hash >> 57) & 0x7F // 하위 7비트 → 제어 태그

- 4. 탐사 순서

  - 1. SIMD를 통해 16개의 제어 바이트 로드
  - 2. 일치하는 태그를 확인합니다(BitMask 사용)
  - 3. 빈/삭제된 슬롯 확인
  - 4. 필요한 경우 다음 그룹으로 이동
  - 5. 성장 정책

  - 하중 계수가 7/8(87.5%)을 초과할 때 증가합니다
  - 묘비가 살아있는 아이템의 절반을 ≥ 때 재탕합니다
  - 빠른 모듈로를 위한 항상 2의 거듭제곱 용량

# Miso - SwissTable HashMap Implementation in Rust

A SwissTable hash map implementation from scratch for learning purposes.

## Overview

**SwissTable** is a modern hash table design developed by Google that optimizes for cache efficiency and SIMD acceleration. This project, called "Miso" (Korean for "young"), implements the core SwissTable algorithms in pure Rust.

### Why SwissTable?

Traditional hash tables use separate chaining or simple open addressing. SwissTable improves upon these with:
- **Metadata-only probing**: Search using only control bytes before touching actual data
- **SIMD acceleration**: Process 16 slots at once using vector instructions
- **Cache-friendly**: Metadata is compact and avoids pointer chasing

---

## Architecture

### File Structure

```
src/
├── lib.rs       # Module declarations
├── control.rs   # Control byte metadata management
├── bitmask.rs   # Bitmask for tracking SIMD matches
├── group.rs     # Generic SIMD operation trait
├── scalar.rs    # Fallback non-SIMD implementation
├── neon.rs      # ARM NEON SIMD implementation
└── table.rs     # Main HashMap implementation
```

---

## Detailed Code Explanation

### control.rs - Metadata Management

#### ControlByte Structure

```rust
#[repr(transparent)]
pub struct ControlByte(u8);
```

Each slot has a **control byte** that stores metadata:

| Value | Binary | Meaning |
|-------|--------|---------|
| `0x80` | `1000_0000` | EMPTY slot (MSB set) |
| `0xFE` | `1111_1110` | DELETED tombstone |
| `0x00-0x7F` | `0xxxxxxx` | FULL (7-bit hash tag, MSB clear) |
| `0xFF` | `1111_1111` | SENTINEL (end of valid slots) |

The **MSB (Most Significant Bit)** distinguishes empty (1) from full (0):
```rust
pub fn is_full(self) -> bool {
    self.0 & 0x80 == 0  // MSB is 0 → slot is occupied
}
```

#### ControlBytes Array

```rust
pub struct ControlBytes {
    bytes: Box<[ControlByte]>,
    capacity: usize,
}
```

Allocates `capacity + 16` bytes with a **sentinel** at position `capacity`:
- Allows SIMD reads to safely read 16 bytes past the end
- Sentinel (`0xFF`) ensures probing stops at boundary

**Clone Indexing** (lines 70-78):
```rust
fn clone_idx(&self, idx: usize) -> Option<usize> {
    if idx >= 15 { return None; }
    Some(self.capacity + idx + 1)  // Mirror indices 0-14
}
```
Mirrors the first 15 control bytes to the overflow area, ensuring SIMD reads that cross the capacity boundary see valid data.

---

### bitmask.rs - Match Tracking

```rust
pub struct BitMask(u16);
```

A 16-bit mask where each bit represents one slot in a SIMD group:

| Operation | Description |
|-----------|-------------|
| `any()` | Check if any bit is set |
| `lsb_idx()` | Get index of least significant set bit |
| `pop_lsb()` | Extract and remove LSB (for iteration) |

**Bit trick** (line 23):
```rust
self.0 &= self.0 - 1;  // Clears the lowest set bit
```

---

### group.rs - SIMD Abstraction

```rust
pub trait GroupOps {
    type View: Copy;
    fn load(ptr: *const ControlByte) -> Self::View;
    fn match_tag(view: Self::View, tag: u8) -> BitMask;
    fn match_deleted(view: Self::View) -> BitMask;
    fn match_empty(view: Self::View) -> BitMask;
}
```

This trait enables **platform-specific optimizations**:
- `NeonOps` - ARM NEON SIMD
- `ScalarOps` - Fallback for platforms without SIMD

---

### scalar.rs - Non-SIMD Fallback

```rust
pub struct ScalarOps;
```

Implements `GroupOps` with simple loops:

```rust
fn match_tag(view: Self::View, tag: u8) -> BitMask {
    let mut mask = BitMask::ZERO;
    for i in 0..16 {
        let byte = ControlByte::from(view[i]);
        if byte.is_full() && byte.tag() == tag {
            mask.set(i);
        }
    }
    mask
}
```

---

### neon.rs - ARM NEON SIMD

```rust
impl GroupOps for NeonOps {
    type View = uint8x16_t;  // 128-bit SIMD register
}
```

**Key operations**:
- `vld1q_u8` - Load 16 bytes in one instruction
- `vceqq_u8` - Parallel comparison of 16 bytes
- `vpaddl_*` - Horizontal reduction to extract bitmask

---

### table.rs - Main HashMap

#### Hash Splitting (lines 180-188)

```rust
fn get_h1_h2_from_key(&self, key: &K) -> (u64, u8) {
    let hash = hasher.finish();
    let h1 = hash >> 7;           // Upper 57 bits for indexing
    let h2 = (hash >> 57) & 0x7F; // Lower 7 bits for metadata
    (h1, h2)
}
```

The 64-bit hash is split:
- **h1**: Primary hash for slot selection
- **h2**: 7-bit tag stored in control byte for quick rejection

#### Probe Sequence (lines 128-172)

```rust
fn probe_for_insert(&self, h1: u64, h2: u8, key: &K) -> Option<InsertProbe> {
    let mut index = h1 & (self.capacity - 1);  // Start position
    loop {
        let group = Group::<DefaultOps>::load(ptr);  // Load 16 bytes
        let tag_mask = group.match_tag(h2);          // Find matching tags
        let delete_mask = group.match_deleted();
        let empty_mask = group.match_empty();

        // Check tag matches for potential key equality
        while let Some(hit) = tag_mask.pop_lsb() {
            if keys_equal(hit) { return Found; }
        }

        // Remember first tombstone for insertion
        if first_tombstone.is_none() && delete_mask.any() {
            first_tombstone = first_delete_position;
        }

        // Empty slot found
        if empty_mask.any() {
            return Vacant(first_tombstone.or(empty_position));
        }

        index = (index + 16) & mask;  // Advance to next group
    }
}
```

#### Growth Policy (lines 244-258)

```rust
fn should_grow(&self) -> bool {
    (tombstones + size + 1) * 8 >= capacity * 7  // Load factor > 7/8
}
```

SwissTable maintains **load factor ≤ 87.5%** for:
- Fewer probes on average
- More empty slots for SIMD to find quickly

---

## Example Usage

```rust
use miso::table::HashMap;

fn main() {
    let mut map = HashMap::new();

    // Insert key-value pairs
    map.insert("name", "Alice");
    map.insert("age", "30");

    // Lookup
    if let Some(name) = map.get(&"name") {
        println!("Name: {}", name);
    }

    // Delete
    map.delete(&"age");
}
```

---

# 원본자료

- **Repository**: https://github.com/thetinygoat/miso
- **Reddit Post**: "Miso: A swiss table implementation from scratch in rust"

### Original Author's Note

> "Hi everyone, excited to share what I've been working on for a couple of weeks. I got interested in implementing a hashmap after reading about them in some detail. I wanted to implement open addressing but after reading about Swiss tables, I decided to dive into the deep end."
>
> "This is my attempt at writing a Swiss table, just for pure learning purposes. It's been fun learning about SIMD and low-level bit manipulation."
>
> **한국어**: 안녕하세요 여러분, 해시맵 구현 프로젝트를 공유하게 되어 기쁩니다. 스위스 테이블에 대해 읽은 후 오픈 어드레싱 대신 스위스 테이블을 직접 구현해보기로 결정했습니다. SIMD와 저수준 비트 조작을 배우는 것이 재미있었습니다.

---

# Current Status

This project is still in its early phases, the core functionality is correct.

- [x] Open addressing with linear probing
- [x] Using control bytes while probing
- [x] Base functionality and correctness
- [x] SIMD acceleration (ARM NEON)
- [ ] x86 AVX2 support
- [ ] Iterator support
- [ ] Sharding for high concurrency

---

## Performance Considerations

### SIMD Benefits
- **16x parallelism**: Compare 16 control bytes in a single instruction
- **Cache efficiency**: Metadata is compact (1 byte per slot)
- **Reduced branches**: Bitmask eliminates per-slot conditional jumps

### Memory Layout
```
+----------------+---------------------+
| Control Bytes  | Key-Value Pairs     |
| (1 byte/slot)  | (variable size)     |
+----------------+---------------------+
```

### Probe Sequence
1. Load 16 control bytes via SIMD
2. Check for matching tags (potential same-key)
3. Check for empty slots (insertion point)
4. Advance to next group if needed

---

## Testing

Run tests with:
```bash
cargo test
```

Run benchmarks:
```bash
cargo bench
```

---

## References

- [SwissTable Design](https://abseil.io/about/design/swisstables)
- [absl::flat_hash_map](https://github.com/abseil/abseil-cpp/blob/master/absl/container/internal/raw_hash_set.h)
- [F14 HashMap](https://github.com/facebook/folly/blob/main/folly/container/F14.md)

