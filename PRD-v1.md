---
repo: claude-radio
rank: 13
score: 0.44
sprint: deferred (re-evaluate after sprint 2 ships)
substrate_anchor: Claude
build_estimate: "4–6 weeks for v0.1"
status: planned
---

# PRD v1.0 — claude-radio

> **One-liner:** Push-to-talk voice loop with any LLM. Ambient mode for dictation; not trying to be a Siri replacement.
>
> **Substrate:** Claude users + ambient/voice interface enthusiasts
> **Launch channels:** r/ClaudeAI, r/LocalLLaMA, AI Twitter, voice-tech newsletters
> **Build estimate (v0.1):** 4–6 weeks for v0.1

---

## What problem does this solve

Voice interfaces with LLMs are clunky. Native voice modes lock you to one vendor. Local solutions are research-grade. claude-radio is a tray app with push-to-talk, multi-provider support (STT and TTS independently), and an ambient/dictation focus rather than trying to be a Siri replacement.

## Why this is a shovel and not a product

Voice-ambient is a known recurring ask. Sovereign by construction. Risky because Anthropic might ship their own. Mitigate by being multi-provider and ambient-focused (different positioning from a Siri replacement).

---

## v0.1 — what ships

Tray app + global hotkey. Push-to-talk → STT (Whisper local or sovereign endpoint) → LLM (any OpenAI-compatible) → TTS (Bulbul, ElevenLabs, or local) → audio response. Designed for ambient/dictation, not conversational.

### Acceptance criteria for v0.1

A v0.1 release is publishable to GitHub when ALL of these are true:

- [ ] Core functionality described above works on the primary developer machine.
- [ ] At least one local-only configuration is documented and tested (no cloud required).
- [ ] BYO endpoint / BYO key configuration is documented.
- [ ] README explains: what it is, who it's for, how to install, how to configure, what it doesn't do.
- [ ] LICENSE present (Apache 2.0 unless overridden).
- [ ] No hardcoded keys or vendor URLs anywhere.
- [ ] No telemetry / phone-home.
- [ ] At least one passing test for the main code path.
- [ ] CI green.
- [ ] AGENTS.md compliance reviewed.

## v0.5 — first major evolution

Wake word support. Continuous mode for note-taking. Voice memo capture with auto-transcription.

## v1.0 — fuller scope

Multi-device sync. Custom voice commands. Skills/plugins.

---

## Architecture sketch

### Stack

Tauri shell. Whisper.cpp for local STT. Pluggable TTS (Bulbul for Indic, ElevenLabs for English, Coqui local).

### Provider abstraction

The shovel MUST expose a provider abstraction even if v0.1 only uses one
provider. Suggested shape:

```
interface Provider {
  name: string;
  endpoint: URL;
  apiKeyEnvVar: string;
  call(input: ProviderInput): Promise<ProviderOutput>;
}
```

The default config in v0.1 must point to a free, local provider where
applicable, and document how to swap in any other.

### Configuration

Configuration order of precedence (highest to lowest):

1. Command-line flags
2. Environment variables (prefix: `CLAUDE_RADIO_*`)
3. User config file (`~/.config/claude-radio/config.toml` on Linux/Mac, equivalent on Windows)
4. Default config (shipped, but never with secrets)

---

## Anti-scope (do NOT build)

Not a Siri replacement. Not a smart home controller. Not a customer-service bot. Ambient AI radio for the desktop.

---

## Tombstone risk and mitigation

**Risk:** Anthropic shipping native voice mode in Claude Desktop. Medium probability — they've hinted at it. Mitigate by being multi-provider and ambient-focused.

**Mitigation:** Ship fast (v0.1 in 4–6 weeks for v0.1). Build community early
(launch on r/ClaudeAI, r/LocalLLaMA, AI Twitter, voice-tech newsletters). Even if upstream absorbs the feature, accumulated
stars and the community are the audience-build payoff.

**Kill signal:** Latency feeling terrible vs native voice tools.

If the kill signal triggers, the maintainer must announce within one week and
either (a) refocus on a remaining gap, (b) merge gracefully into upstream if
they're receptive, or (c) mark the repo as archived with a clear pointer to the
replacement.

---

## Launch plan

### Pre-launch checklist

- [ ] Repo on GitHub at `github.com/sovereign-shovels/claude-radio`
- [ ] README polished (see template in `_templates/`)
- [ ] At least 3 issues / discussions seeded (real ones, not placeholder)
- [ ] LICENSE, CODE_OF_CONDUCT, CONTRIBUTING present
- [ ] Demo asset (gif, screenshot, or short video — depending on category)
- [ ] First-launch post drafted for primary launch channel

### Day-1 launch

Post to: r/ClaudeAI, r/LocalLLaMA, AI Twitter, voice-tech newsletters

Subject template (adjust per channel):
- Show HN: `Show HN: claude-radio – Push-to-talk voice loop with any LLM. Ambient mode for dictation; not trying to be a Siri replacement.`
- Reddit: `[OSS] Push-to-talk voice loop with any LLM. Ambient mode for dictation; not trying to be a Siri replacement.` with full post explaining the gap and the build
- Twitter/X: thread leading with the demo gif

### Week-1 follow-up

- Respond to every issue and comment within 24h.
- Ship at least one bugfix release based on launch feedback.
- Cross-post to secondary channels.

### Month-1 review

- Assess star velocity and community formation.
- If kill signal triggered, follow tombstone protocol above.
- If trajectory is healthy, plan v0.5.

---

## Cross-references

- Constitution: [[AGENTS]]
- Public README: [[README]]
- Progress frontmatter: [[progress]]
- Internal knowledge graph: [[knowledge-graph]]
