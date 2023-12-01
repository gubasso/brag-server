# implement_api_endpoints (Issue)

**endpoints**

- [x] `/`: array of repositories
- [ ] review axum middleware
- [x] `/count`: count commits for each repo[^1]
  - [ ] get enum utils from ripissue: enum to lowercase
  - `/count?by=day`
  - `/count?by=week`
  - `/count?by=month&author_email=gubasso@cwnt.io`
- `/<host>`: array of repos
- `/<host>/<user>`: array of repos
- `/<host>/<user>/<name>`: array of commits of that repo[^1]

[^1]: query string `author_email`: filter commits
