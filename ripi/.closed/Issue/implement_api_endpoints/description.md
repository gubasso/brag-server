# implement_api_endpoints (Issue)

**endpoints**

- [x] `/`: array of repositories
- [x] review axum middleware
- [x] `/count`: count commits for each repo[^1]
  - [x] enum filter query-string lower case
  - `/count?by=day`
  - `/count?by=week`
  - `/count?by=month&author_email=gubasso@cwnt.io`
  - `/count?full_name=gubasso/dwm`
  - [x] implement the req query to the sql query
  - [x] `/count?author_email=gubasso`
