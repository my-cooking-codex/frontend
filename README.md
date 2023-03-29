# My Cooking Codex - Frontend

## Without Docker
### Requirements
- Web server capable of serving static content
- Rust Stable (2021 edition)
- Trunk
- PNPM

### Build
Run these commands:

```
pnpm install

trunk build --release
```

Copy generated files in `./dist` and place on web server.
