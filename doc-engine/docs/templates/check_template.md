# Check Template

**Audience**: Developers

Template for defining new compliance checks.

## Check Definition (rules.toml)

```toml
[[rules]]
id = <next_id>
category = "<category>"
description = "<description>"
severity = "error|warning|info"
type = "file_exists|dir_exists|builtin"
path = "<path>"        # for declarative types
handler = "<handler>"  # for builtin type
```

## Builtin Handler Template

```rust
pub struct MyCheck {
    pub def: RuleDef,
}

impl CheckRunner for MyCheck {
    fn id(&self) -> CheckId { CheckId(self.def.id) }
    fn category(&self) -> &str { &self.def.category }
    fn description(&self) -> &str { &self.def.description }

    fn run(&self, ctx: &ScanContext) -> CheckResult {
        // Implementation here
        CheckResult::Pass
    }
}
```
