# Result

```bash
map : {"count": 1}
map : {"count": 13}
```

# `pub fn or_insert(self, default: V)-> &'a mut V`

- https://doc.rust-lang.org/stable/std/collections/hash_map/enum.Entry.html#method.or_insert

- Ensures a value is in the entry by inserting the default if empty, and returns a mutable reference to the value in the entry.
  - 기본값이 비어 있는 경우 항목에 값을 삽입하여 값이 포함되도록 하고, 항목의 값에 대한 변경 가능한 참조를 반환합니다.

