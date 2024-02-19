# ripissue drafts

## now

- [ ] server is not updating correctly

## backlog

- [ ] cleanup: review project files and scripts
- [ ] add/update changelog
- [ ] readme: install steps
- [ ] update main cwnt-io repo
- [ ] implement query string filter `repo` in `count` endpoint
  - e.g. `/count?repo=gubasso/ripissue`
- migrate from `commit-json` script to native git2 rust lib
  - remove the following conditional
  ```rs
  if n_commits.parse::<i32>().unwrap() = 1 {
  ```
