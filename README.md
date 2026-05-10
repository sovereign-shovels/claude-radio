# claude-radio

> Push-to-talk voice loop with any LLM. Ambient mode for dictation.

**Status:** v0.1 — ready to use.

**Sovereignty:** sovereign-by-construction. Works with local Ollama by default.

---

## Install

```bash
git clone https://github.com/sovereign-shovels/claude-radio.git
cd claude-radio
cargo build --release
```

## Usage

```bash
# Generate a podcast script from an RSS feed
claude-radio generate --feed https://news.ycombinator.com/rss --output script.txt

# The script can then be spoken via bulbul-studio or any TTS tool
```

---

## License

Apache 2.0. See [LICENSE](./LICENSE).

## Part of sovereign-shovels

This repo is part of the [sovereign-shovels](https://github.com/sovereign-shovels) portfolio.
