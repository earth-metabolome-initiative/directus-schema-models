# Time Report for Directus Schema Generation

The total time spent on all tasks was now.
The slowest task was `DB Introspection` which took 5 seconds, 953 ms, 825 µs and 828 ns (94.08% of all time).

| name                     | time                                 | percentage | comment |
|--------------------------|--------------------------------------|------------|---------|
| DB Connection            | 6 ms, 348 µs and 462 ns              | 0.10%      |         |
| DB Introspection         | 5 seconds, 953 ms, 825 µs and 828 ns | 94.08%     |         |
| SQL Workspace Generation | 151 ms, 113 µs and 550 ns            | 2.39%      |         |
| Code Formatting          | 181 ms, 503 µs and 62 ns             | 2.87%      |         |
| TOML Formatting          | 35 ms, 620 µs and 867 ns             | 0.56%      |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `writing_crate_lib` which took 136 ms, 64 µs and 305 ns (90.04% of all time).

| name                    | time                     | percentage | comment |
|-------------------------|--------------------------|------------|---------|
| writing_crate_toml      | 6 ms, 848 µs and 859 ns  | 4.53%      |         |
| writing_crate_lib       | 136 ms, 64 µs and 305 ns | 90.04%     |         |
| writing_sink_crate_toml | 1 ms, 445 µs and 348 ns  | 0.96%      |         |
| writing_sink_crate_lib  | 3 ms, 29 µs and 447 ns   | 2.00%      |         |
| workspace_toml          | 3 ms, 646 µs and 21 ns   | 2.41%      |         |
| workspace_rustfmt       | 79 µs and 570 ns         | 0.05%      |         |

![Plot](TIME_REQUIREMENTS.png)
