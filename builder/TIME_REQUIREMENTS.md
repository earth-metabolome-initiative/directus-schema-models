# Time Report for Directus Schema Generation

The total time spent on all tasks was 2 minutes.
The slowest task was `Clippy Linting` which took 1 minute, 29 seconds, 681 ms, 973 µs and 454 ns (93.02% of all time).

| name                     | time                                            | percentage |
|--------------------------|-------------------------------------------------|------------|
| DB Connection            | 6 ms, 501 µs and 953 ns                         | 0.01%      |
| DB Introspection         | 6 seconds, 184 ms, 863 µs and 635 ns            | 6.42%      |
| SQL Workspace Generation | 184 ms, 276 µs and 747 ns                       | 0.19%      |
| Code Formatting          | 202 ms, 675 µs and 626 ns                       | 0.21%      |
| TOML Formatting          | 150 ms, 684 µs and 188 ns                       | 0.16%      |
| Clippy Linting           | 1 minute, 29 seconds, 681 ms, 973 µs and 454 ns | 93.02%     |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `writing_crate_lib` which took 162 ms, 983 µs and 318 ns (88.45% of all time).

| name                    | time                      | percentage |
|-------------------------|---------------------------|------------|
| writing_crate_toml      | 7 ms, 991 µs and 833 ns   | 4.34%      |
| writing_crate_lib       | 162 ms, 983 µs and 318 ns | 88.45%     |
| writing_sink_crate_toml | 1 ms, 697 µs and 61 ns    | 0.92%      |
| writing_sink_crate_lib  | 7 ms, 280 µs and 753 ns   | 3.95%      |
| workspace_toml          | 4 ms, 231 µs and 372 ns   | 2.30%      |
| workspace_rustfmt       | 92 µs and 410 ns          | 0.05%      |
