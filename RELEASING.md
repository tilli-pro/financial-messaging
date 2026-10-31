# Releasing

Crates are published in this order: `iso-8583`, `iso-20022-codegen`, `iso-20022`.
All crates share one workspace version and one tag `vX.Y.Z`.

## One-time first publish (owner, by hand)

Trusted Publishing can only be configured on a crate that already exists, so the
first release is manual.

1. On crates.io create an API token scoped to `publish-new` and `publish-update`
   (crate scope `iso-*` if offered, short expiry).
2. `cargo login` and paste the token.
3. From a clean checkout of `main` at the `0.0.1` commit, publish in order:
   ```sh
   cargo publish -p iso-8583 --locked
   cargo publish -p iso-20022-codegen --locked
   cargo publish -p iso-20022 --locked
   ```
4. Add the team as owner of each crate:
   `cargo owner --add github:tilli-pro:financial-messaging-maintainers <crate>`
5. On each crate's crates.io Settings page add a Trusted Publisher: repository
   `tilli-pro/financial-messaging`, workflow `release.yml`, environment `release`.
6. In GitHub, Settings > Environments, create environment `release` with at
   least one required reviewer.
7. Do not tag `v0.0.1` afterwards: it is already published by hand, so the
   workflow would fail at publish. The first workflow-driven release is the
   next version.
8. Revoke the API token (and run `cargo logout`).

## Routine release

1. In one PR, bump `[workspace.package] version` in `Cargo.toml`, refresh
   `Cargo.lock`, and move the `Unreleased` changelog entries under a new
   `## [x.y.z] - date` heading.
2. Get it reviewed and merge.
3. Tag the merge commit and push the tag:
   ```sh
   git switch main && git pull
   git tag v0.x.y && git push origin v0.x.y
   ```
4. The `Release` workflow verifies the tag equals the workspace version, runs
   tests and clippy, waits for approval on the `release` environment, then
   publishes all three crates through a short-lived OIDC token.

The version bump must be in the very commit the tag points at: the workflow
fails if the tag and `Cargo.toml` disagree, and crates.io versions can never be
re-published, only yanked.
