# ripissue drafts

## roadmap

- [ ] deploy steps
  - [x] learn make? make file?
  - [ ] deploy with make: try to make as automatic as possible
    - check if asdf is already installed install_asdf
      - same for rust and pyhton
    - load_db and brag-server: compile, cp binaries to .local/bin, and run them there
  - [ ] setup NPM with `host.docker.internal`
  - [x] server: setup nginx / nginx proxy manager
  - [x] migrate asdf to script
    - ignore .tool-versions
- initial server setup (spin up)
  - systemctl inside rust?

## done

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

## definitions

- run caddy in host too:
  - it will serve all of the docker services and other services behind
- rust will run in the host: it calls docker to run mergestat
  - i can adapt it later
