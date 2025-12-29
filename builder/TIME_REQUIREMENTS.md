# Time Report for Directus Schema Generation

The total time spent on all tasks was now.
The slowest task was `DB Introspection` which took 6 seconds, 99 ms, 971 µs and 14 ns (93.91% of all time).

| name                     | time                               | percentage | comment |
|--------------------------|------------------------------------|------------|---------|
| DB Connection            | 6 ms, 276 µs and 102 ns            | 0.10%      |         |
| DB Introspection         | 6 seconds, 99 ms, 971 µs and 14 ns | 93.91%     |         |
| SQL Workspace Generation | 169 ms, 17 µs and 873 ns           | 2.60%      |         |
| Code Formatting          | 197 ms, 85 µs and 996 ns           | 3.03%      |         |
| TOML Formatting          | 22 ms, 941 µs and 543 ns           | 0.35%      |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `writing_crate_lib` which took 156 ms, 995 µs and 960 ns (92.89% of all time).

| name               | time                      | percentage | comment |
|--------------------|---------------------------|------------|---------|
| writing_crate_toml | 7 ms, 657 µs and 455 ns   | 4.53%      |         |
| writing_crate_lib  | 156 ms, 995 µs and 960 ns | 92.89%     |         |
| workspace_toml     | 4 ms, 272 µs and 278 ns   | 2.53%      |         |
| workspace_rustfmt  | 92 µs and 180 ns          | 0.05%      |         |

![Plot](TIME_REQUIREMENTS.png)
