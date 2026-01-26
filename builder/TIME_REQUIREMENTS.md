# Time Report for Directus Schema Generation

The total time spent on all tasks was 2 minutes.
The slowest task was `Clippy Linting` which took 1 minute, 40 seconds, 934 ms, 378 µs and 353 ns (93.64% of all time).

| name                     | time                                            | percentage |
|--------------------------|-------------------------------------------------|------------|
| DB Connection            | 15 ms, 492 µs and 672 ns                        | 0.01%      |
| DB Introspection         | 6 seconds, 166 ms, 289 µs and 957 ns            | 5.72%      |
| SQL Workspace Generation | 223 ms, 237 µs and 126 ns                       | 0.21%      |
| Code Formatting          | 228 ms, 423 µs and 629 ns                       | 0.21%      |
| TOML Formatting          | 220 ms, 401 µs and 340 ns                       | 0.20%      |
| Clippy Linting           | 1 minute, 40 seconds, 934 ms, 378 µs and 353 ns | 93.64%     |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `writing_crate_lib` which took 188 ms, 354 µs and 602 ns (84.37% of all time).

| name                    | time                      | percentage |
|-------------------------|---------------------------|------------|
| writing_crate_toml      | 18 ms, 655 µs and 390 ns  | 8.36%      |
| writing_crate_lib       | 188 ms, 354 µs and 602 ns | 84.37%     |
| writing_sink_crate_toml | 1 ms, 963 µs and 944 ns   | 0.88%      |
| writing_sink_crate_lib  | 8 ms, 55 µs and 479 ns    | 3.61%      |
| workspace_toml          | 6 ms, 102 µs and 121 ns   | 2.73%      |
| workspace_rustfmt       | 105 µs and 590 ns         | 0.05%      |
