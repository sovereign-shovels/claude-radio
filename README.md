# claude-radio

> Push-to-talk voice loop with any LLM. Ambient mode for dictation.

**Status:** v0.1 — ready to use.

**Sovereignty:** sovereign-by-construction. Works with local Ollama by default.

---

## Architecture

```
┌─────────────┐     ┌──────────────┐     ┌─────────────────┐
│   RSS feed  │────▶│   claude-    │────▶│   Ollama /      │
│  (HN, etc.) │     │   radio      │     │   Claude API    │
└─────────────┘     │  (fetch +    │     │   (script gen)  │
                    │   prompt)    │     └─────────────────┘
                    └──────────────┘              │
                           │                      ▼
                           ▼               ┌──────────────┐
                    ┌──────────────┐       │ podcast-     │
                    │  podcast-    │       │ script.txt   │
                    │  script.txt  │       │ (TTS input)  │
                    └──────────────┘       └──────────────┘
```

## What this is

Turn your RSS feeds into a daily podcast script. Fetch headlines, generate a briefing with any LLM, save to text. Then pipe through bulbul-studio or any TTS for audio.

## What this isn't

- Not a TTS tool (see [bulbul-studio](https://github.com/sovereign-shovels/bulbul-studio))
- Not a music/radio streaming app
- Not a real-time news aggregator

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

# Use a local model
claude-radio generate --feed https://hnrss.org/newest --model gemma4:latest --endpoint http://localhost:11434/v1/chat/completions

# The script can then be spoken via bulbul-studio or any TTS tool
```

**Demo output:**
```
$ claude-radio generate --feed https://hnrss.org/newest?points=100 \
  -o /tmp/radio-test.txt -m gemma4:latest
Fetching RSS...
Generating script...
Saved script to /tmp/radio-test.txt
```

---

## License

Apache 2.0. See [LICENSE](./LICENSE).

## Part of sovereign-shovels

This repo is part of the [sovereign-shovels](https://github.com/sovereign-shovels) portfolio.
