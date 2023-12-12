# deploy_with_systemd_units (Issue)

binaries to systemd

app files to: `/opt/brag-server/`

chmod +x ./scripts/*

- [ ] units with fake env variables
- [ ] env.sh -> .env
- [ ] cp binaries, units, and env_file

cp binaries to `/opt/brag-server`
cp to -> `/etc/systemd/system/YourServiceName.service`
`/opt/brag-server/env`

- <user_name> as a env var, depending on dev/prod

sudo systemctl enable spin_db.service --now
sudo systemctl enable load_db@<user_name>.service --now
sudo systemctl enable brag-server@<user_name>.service --now

