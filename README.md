# Scorched

A simple logging library for scorching all those pesky bugs.

## Example

This example shows how to use the library to log a message and check optional values.

```rust
use scorched::*;

let something = Some(5);
let nothing = None::<i32>;

something.log_except(LogImportance::Error, "This should not be logged");
nothing.log_except(LogImportance::Error, "This should be logged");

log_this(LogData {
    LogImportance::Debug,
    "All of the tests have now finished!"
});
```