# Below is a tiny, educational SwissTable-style hash map, stripped down to the core ideas:

- control bytes
- open addressing
- partial hash (fingerprint)
- group scanning (SIMD-like, but scalar for clarity)

- ⚠️ This is not production code. It is intentionally small so you can see the mechanism.
