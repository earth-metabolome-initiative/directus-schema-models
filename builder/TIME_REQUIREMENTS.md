# Time Report for Directus Schema Generation

The total time spent on all tasks was now.
The slowest task was `DB Introspection` which took 6 seconds, 105 ms, 492 µs and 920 ns (93.66% of all time).

| name                     | time                                 | percentage | comment |
|--------------------------|--------------------------------------|------------|---------|
| DB Connection            | 5 ms, 696 µs and 45 ns               | 0.09%      |         |
| DB Introspection         | 6 seconds, 105 ms, 492 µs and 920 ns | 93.66%     |         |
| SQL Workspace Generation | 174 ms, 192 µs and 157 ns            | 2.67%      |         |
| Code Formatting          | 200 ms, 129 µs and 976 ns            | 3.07%      |         |
| TOML Formatting          | 33 ms, 343 µs and 608 ns             | 0.51%      |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `writing_crate_lib` which took 156 ms, 972 µs and 387 ns (90.11% of all time).

| name                    | time                      | percentage | comment |
|-------------------------|---------------------------|------------|---------|
| writing_crate_toml      | 7 ms, 648 µs and 446 ns   | 4.39%      |         |
| writing_crate_lib       | 156 ms, 972 µs and 387 ns | 90.11%     |         |
| writing_sink_crate_toml | 1 ms, 726 µs and 234 ns   | 0.99%      |         |
| writing_sink_crate_lib  | 3 ms, 518 µs and 57 ns    | 2.02%      |         |
| workspace_toml          | 4 ms, 245 µs and 729 ns   | 2.44%      |         |
| workspace_rustfmt       | 81 µs and 304 ns          | 0.05%      |         |

![Plot](TIME_REQUIREMENTS.png)
