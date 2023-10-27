# My Cooking Codex - Frontend

## Without Docker
### Requirements
- Web server capable of serving static content
- Rust Stable (2021 edition)
- Trunk
- npm

### Build
Run these commands:

```
npm ci

trunk build --release
```

Copy generated files in `./dist` and place on web server.
