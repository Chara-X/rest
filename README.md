# rest

## Example

```rust
rest::client!({
    info {
        title: "User management",
        version: "1.0",
        description:
            "Just for example",
    };
    server {
        url: "/api/v1",
    };
    /// Create a new user
    op("POST", "/users", msg::User, ());
    /// Get a user
    op("GET", "/users/{id}", (), msg::User);
});
```
