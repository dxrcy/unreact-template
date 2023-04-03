# Unreact App Template

A basic template for an [Unreact](https://crates.io/crates/unreact) SSG app in Rust.

-   [Live on GitHub Pages](https://darccyy.github.io/unreact-template)
-   [Unreact documentation](https://docs.rs/unreact/latest/unreact)
-   [Unreact template source](https://github.com/darccyy/unreact-template)

This template uses the latest stable version of Unreact.

# Usage

```bash
# Clone the repo
git clone https://github.com/darccyy/unreact-template my-unreact-app
cd my-unreact-app

# Run in dev mode
# Serves to localhost:3000, reloads client on file changes
just serve

# Compile in production mode
# This is in the GitHub action
cargo run --no-default-features
```

## GitHub Pages Setup

GitHub will automatically build to the `gh-pages` branch, with the `build.yaml` Action.
The action will automatically run when the repository is initially cloned.

In repository settings, navigate to the `Pages` tab, and change 'Branch' to `gh-pages`.
GitHub pages will automatically update, and the website should be live soon.

![Unreact Icon](./public/icon.png)
