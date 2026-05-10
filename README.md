# claude-radio

> Push-to-talk voice loop with any LLM. Ambient mode for dictation; not trying to be a Siri replacement.

**Status:** v0.1 — planning. Not yet released.

**Sovereignty:** sovereign-by-construction. BYO endpoint, BYO key, BYO model.
A local-only configuration is documented and tested.

This is a community project, **not affiliated with Claude**.
Best-effort community shovel — no SLA, no roadmap commitments.

---

## What this is

Push-to-talk voice loop with any LLM. Ambient mode for dictation; not trying to be a Siri replacement.

## What this isn't

Not a Siri replacement. Not a smart-home controller. Not a voice agent for customer support.

## Install

> Coming with v0.1 release.

## Configure

You bring the model. By default `claude-radio` tries to use a local provider:

- For LLM endpoints: Ollama at `http://localhost:11434`
- For voice endpoints: configurable, see [docs/configure.md]

To use any other provider (Claude, GPT, Hermes, OpenRouter, Sarvam, etc.):

```toml
# ~/.config/claude-radio/config.toml
[provider]
endpoint = "https://api.your-provider.com/v1"
api_key_env = "YOUR_PROVIDER_KEY"
model = "your-model-name"
```

Anthropic, OpenAI, and Sarvam endpoints all work. Local Ollama, llama.cpp,
LM Studio, and vLLM all work via their OpenAI-compatible endpoints.

## Why this exists

Voice interfaces with LLMs are clunky. Native voice modes lock you to one vendor. Local solutions are research-grade. claude-radio is a tray app with push-to-talk, multi-provider support (STT and TTS independently), and an ambient/dictation focus rather than trying to be a Siri replacement.

## What's next

See [PRD-v1.md](./PRD-v1.md) for the full v0.1 → v0.5 → v1.0 plan.

## License

Apache 2.0. See [LICENSE](./LICENSE).

## Part of sovereign-shovels

This repo is part of the [sovereign-shovels](https://github.com/sovereign-shovels)
portfolio of small, focused, sovereign-by-construction AI utilities.

Other shovels: claude-vault, bulbul-studio, saaras-tray, claude-prompts,
ollama-cron, mcp-forge, sarvam-pdf, agent-console, sarvam-meet, obsidian-llm,
llm-diff, claude-bridge, claude-radio, sarvam-cast.
