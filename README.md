# SecretMap Operator

## Intended features

> consider renaming to SecretMap (secretmap, secretmaps, sm)

- Multiple secret sources (clouds, platforms)
- Allow static secrets for non-sensitive values that must be in the same secret
- Allow adding to existing secrets (with a matchLabels for safey)
- Schema and documentations generated from code (single source of truth)
- extract values from JMES Path
- Support rotation (separate CustomResource)

For now, see <example.yml> for an idea of what a SecretMap coule look like.
