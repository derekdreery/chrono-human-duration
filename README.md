# chrono-human-duration

Very small crate for format durations for humans.

## Example

```rust
use chrono_human_duration::ChronoHumanDuration;
use chrono::Duration;

let d = Duration::weeks(2) + Duration::days(3);
assert_eq!(d.format_human().to_string(), "2 weeks ago");
let d = Duration::minutes(20);
assert_eq!(d.format_human().to_string(), "just now");
```
