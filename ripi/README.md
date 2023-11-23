# ripissue drafts

## roadmap

- [x] command_to_clone_repository

- toml config
  - load ssh-agent (keychain as a dependency)
  - clone repositories
  - update repos routine (push)
  - from repo to json
  - filter and aggregate data
  - save to db
- initial server setup (spin up)
  - systemctl inside rust?
- server up, serving routes and repo updates routines
- Follow examples at: https://github.com/tokio-rs/axum/tree/main/examples

- needs to be authenticated (e.g. ssh-agent) if is needed for git clone. This program will not handle this.
