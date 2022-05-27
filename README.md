# SecretMap Operator

***Warning: this is a new project and is not currently in a usable state. Check back soon.***

> todo: description. who is it for and what does it solve? diagram

## Intended features

- [ ] Multiple secret sources (clouds, platforms)
- [ ] Allow static secrets for non-sensitive values that must be in the same secret
- [ ] Allow adding to existing secrets (with a matchLabels for safety)
- [x] Schema with documentated fields, and example manifests generated from code (single source of truth)
- [ ] Extract values from JMES Path
- [ ] Support rotation (separate CustomResource)

See [auto-generated example manifests](./manifests/examples/) for usage.
## References

- [Reference Controller](https://github.com/kube-rs/controller-rs)
- [Another Controller](https://github.com/Pscheidl/rust-kubernetes-operator-example)
