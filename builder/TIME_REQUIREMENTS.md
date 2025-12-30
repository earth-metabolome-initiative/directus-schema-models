# Time Report for Directus Schema Generation

The total time spent on all tasks was 2 minutes.
The slowest task was `Clippy Linting` which took 1 minute, 25 seconds, 9 ms, 10 µs and 747 ns (92.88% of all time).

| name                     | time                                         | percentage | comment |
|--------------------------|----------------------------------------------|------------|---------|
| DB Connection            | 6 ms, 382 µs and 723 ns                      | 0.01%      |         |
| DB Introspection         | 6 seconds, 76 ms, 931 µs and 252 ns          | 6.64%      |         |
| SQL Workspace Generation | 181 ms, 800 µs and 576 ns                    | 0.20%      |         |
| Code Formatting          | 194 ms, 288 µs and 208 ns                    | 0.21%      |         |
| TOML Formatting          | 62 ms, 48 µs and 517 ns                      | 0.07%      |         |
| Clippy Linting           | 1 minute, 25 seconds, 9 ms, 10 µs and 747 ns | 92.88%     |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `writing_crate_lib` which took 161 ms, 13 µs and 866 ns (88.57% of all time).

| name                    | time                     | percentage | comment |
|-------------------------|--------------------------|------------|---------|
| writing_crate_toml      | 7 ms, 444 µs and 271 ns  | 4.09%      |         |
| writing_crate_lib       | 161 ms, 13 µs and 866 ns | 88.57%     |         |
| writing_sink_crate_toml | 1 ms, 784 µs and 823 ns  | 0.98%      |         |
| writing_sink_crate_lib  | 7 ms, 179 µs and 729 ns  | 3.95%      |         |
| workspace_toml          | 4 ms, 293 µs and 440 ns  | 2.36%      |         |
| workspace_rustfmt       | 84 µs and 447 ns         | 0.05%      |         |

![Plot](TIME_REQUIREMENTS.png)
