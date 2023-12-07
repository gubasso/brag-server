# ripissue drafts

## roadmap

- [ ] deploy steps
  - [x] migrate asdf to script
    - ignore .tool-versions
  - compose:
    - [ ] rust app:
      - https://www.docker.com/blog/simplify-your-deployments-using-the-rust-official-image/
    - load-balancer/web-server:
      - [ ] study caddy youtube
      - https://www.docker.com/blog/deploying-web-applications-quicker-and-easier-with-caddy-2/
    - postgresql server

  - container using a ssh-agent
    - https://www.jamesridgway.co.uk/sharing-an-ssh-agent-between-a-host-machine-and-a-docker-container/
  
  - initial server setup (spin up)
    - systemctl inside rust?
  - `brag-server` as a docker container (to be deployed with a `docker run` or `docker compose`)
  - `nginx` as a docker container (`compose`)
- [x] refactor error handling: return errors like python-eve: as http and also as a json with error details
- [x] routine to update repos periodically
- [x] implement migrate! to main, to reset db for a new binary run
- [x] setup axum structure
- [x] save everything in db
- [x] get all pub repos
  - [x] api requests
- [x] cloning repositories
- [x] server config with keychain
- [x] document keychain as a dependency, if ssh-agent is needed (server setup)
  - https://wiki.archlinux.org/title/GnuPG#gpg-agent
  - [funtoo / keychain - Suggest alternatives (use gpg-agent systemd unit) #138](https://github.com/funtoo/keychain/issues/138)
- [x] command_to_clone_repository
- [x] endpoint filter by user email:
