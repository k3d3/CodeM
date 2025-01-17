# Codem Config File Format

The config file uses TOML format to configure a Codem MCP server. The config specifies:
- Projects that can be managed
- Where session data is stored
- What commands are allowed

## Projects

Each project is defined as:

```toml
[[projects]]
# Name used to identify the project when creating sessions
name = "my-project"

# Root directory for the project - operations are relative to this
base_path = "/path/to/project"

# Optional: List of additional allowed directory paths
allowed_paths = [
    "/path/to/other/allowed/dir",
    "/path/to/other/allowed/dir2"
]

# Optional: Command to run tests for this project
test_command = "cargo test"
```

The project name is used when calling create_session from a client. For example, if you've defined a project with `name = "my-project"`, create a session by calling:

```json
{
    "name": "create_session",
    "arguments": {
        "project": "my-project"
    }
}
```

Projects must have unique names. Multiple projects can be defined by repeating the `[[projects]]` section.

## Session Storage

```toml
# Path where session data will be stored 
# Parent directory must exist
session_file = "/path/to/sessions/sessions.toml"
```

## Command Patterns

Two lists of regular expressions define which commands can be executed:

```toml
# Commands that match safe_patterns can be executed without confirmation
# (as long as they don't also match risky_patterns)
safe_patterns = [
    "^grep -r \".*\" .*$",    # Allow recursive grep
    "^ls( -[a-zA-Z]+)?$",     # ls with optional flags
    "^echo [a-zA-Z0-9_-]+$"   # Basic echo commands
]

# Commands that match risky_patterns require extra confirmation
risky_patterns = [
    "^rm .*",            # Any rm command 
    "^git (push|pull)"   # Git remote operations
]
```

All patterns:
- Must be valid regular expressions
- Are matched against the full command string
- Cannot be empty

A command is considered:
- Safe: If it matches at least one safe_pattern and no risky_patterns
- Risky: If it matches any risky_pattern
- Invalid: If it matches no patterns

See example.toml for a complete example configuration.

## Implementation Notes

While this TOML maps to a simple list of projects, internally Codem:
- Stores projects in a HashMap for efficient lookups
- Wraps projects in Arc for thread-safe sharing
- Validates all paths and patterns
- Caches compiled regex patterns

The TOML format is designed to be user-friendly while allowing these optimizations internally.