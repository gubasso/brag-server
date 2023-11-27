# ripissue drafts

## roadmap

- [x] get all pub repos
  - [x] api requests
- [x] cloning repositories
- [x] server config with keychain
- [x] document keychain as a dependency, if ssh-agent is needed (server setup)
  - https://wiki.archlinux.org/title/GnuPG#gpg-agent
  - [funtoo / keychain - Suggest alternatives (use gpg-agent systemd unit) #138](https://github.com/funtoo/keychain/issues/138)
- [x] command_to_clone_repository

- toml config
  - update repos routine (push)
  - from repo to json
  - filter and aggregate data
  - save to db
- sqlx migration details: https://docs.rs/sqlx/latest/sqlx/macro.migrate.html
- initial server setup (spin up)
  - systemctl inside rust?
- server up, serving routes and repo updates routines
- Follow examples at: https://github.com/tokio-rs/axum/tree/main/examples
- `brag-server` as a docker container (to be deployed with a `docker run` or `docker compose`)
- `nginx` as a docker container (`compose`)

## definitions

- needs to be authenticated (e.g. ssh-agent) if is needed for git clone. This program will not handle this.
