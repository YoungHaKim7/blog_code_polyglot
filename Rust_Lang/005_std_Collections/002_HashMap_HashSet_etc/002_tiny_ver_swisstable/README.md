# Below is a tiny, educational SwissTable-style hash map, stripped down to the core ideas:

- control bytes
- open addressing
- partial hash (fingerprint)
- group scanning (SIMD-like, but scalar for clarity)

- ⚠️ This is not production code. It is intentionally small so you can see the mechanism.

# 1. What we are going to build (scope)
- 1. 우리가 만들 것 (범위)

- We will implement:
  - `TinySwissMap<K, V>`
  - Fixed capacity (power of two)
  - Linear probing in groups
  - Control bytes (`EMPTY`, `FULL`)
  - Partial hash (`h2`)
  - Separate control array + entry array

- We will not implement:
  - resizing
  - deletion
  - real SIMD (we simulate it with loops)

- This mirrors the principle, not the full complexity.

- 구현하겠습니다:
  - `TinySwissMap<K, V>`
  - 고정 용량(2의 거듭제곱)
  - 그룹별 선형 탐색
  - 제어 바이트('EMPTY', 'FULL')
  - 부분 해시 ('h2')
  - 별도 제어 배열 + 항목 배열

- 우리는 실행하지 않을 것입니다:
  - 크기 조정
  - 삭제
  - 실제 SIMD (루프로 시뮬레이션합니다)

- 이것은 전체 복잡성이 아니라 원칙을 반영합니다.


