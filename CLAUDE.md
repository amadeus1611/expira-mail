# expira-mail

GSSC office stack for gsscph.com: Stalwart (mail, calendar, contacts, WebDAV files) and Garage (S3 storage), both Rust, via Docker Compose.

- Private repo. Never commit `.env`, `data/`, passwords or client data.
- Pin exact image tags; verify upstream before bumping.
- CI: `shellcheck scripts/*.sh` and `docker compose config -q` must pass.
- No invented figures (prices, quotas): cite a source or say unverified.
