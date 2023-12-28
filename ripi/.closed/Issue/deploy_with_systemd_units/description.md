# deploy_with_systemd_units (Issue)

new approach: run everything using just docker/docker-compose

- [x] deploy at production
- [x] prune docker
- [x] run d compose with load_db
- [x] add brag-server to d compose
- [x] refactor load_db: remove mergestat depedency


---

- remove `env`
- load `env_vars.sh`
- `setup_initial.sh`
  - mkdir `/opt/brag-server`
  - save env_file
  - install asdf / plugins / runners / versions
- `.runner-dev-ok`
- binaries to systemd
- app files to: `/opt/brag-server/`
- chmod +x ./scripts/*
- units with env variables
  - EnvironmentFile=/opt/brag-server/env
- env.sh -> .env
- cp binaries, units, and env_file
- cp binaries to `/opt/brag-server`
- cp to -> `/etc/systemd/system/YourServiceName.service`
- `/opt/brag-server/env`
- <user_name> as a env var, depending on dev/prod
- sudo systemctl enable spin_db.service --now
- sudo systemctl enable load_db@<user_name>.service --now
- sudo systemctl enable brag-server@<user_name>.service --now
