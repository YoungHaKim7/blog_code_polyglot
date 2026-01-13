# Result

```bash
$ cargo nextest run --nocapture
────────────
    Starting 4 tests across 2 binaries
       START             json_serde_impl::serde_deserde_test deserde_invalid_json

running 1 test
test deserde_invalid_json ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

        PASS [   0.323s] json_serde_impl::serde_deserde_test deserde_invalid_json
       START             json_serde_impl::serde_deserde_test input_json

running 1 test
RawJsonDeserde {
    title: "optio molestias id quia eum",
    value: "quo et expedita modi cum officia vel magni\ndoloribus qui repudiandae\nvero nisi sit\nquos veniam quod sed accusamus veritatis error",
}
test input_json ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

        PASS [   0.374s] json_serde_impl::serde_deserde_test input_json
       START             json_serde_impl::serde_deserde_test serde_data

running 1 test
test serde_data ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

        PASS [   0.374s] json_serde_impl::serde_deserde_test serde_data
       START             json_serde_impl::serde_deserde_test serde_simple

running 1 test
test serde_simple ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

        PASS [   0.323s] json_serde_impl::serde_deserde_test serde_simple
────────────
     Summary [   1.402s] 4 tests run: 4 passed, 0 skipped

```

