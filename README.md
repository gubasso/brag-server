# brag-server

<!-- toc -->

- [Usage](#usage)
  - [Requests examples](#requests-examples)
- [Server Deployment ("install")](#server-deployment-install)
  - [Prerequisites](#prerequisites)
  - [Deploy](#deploy)

<!-- tocstop -->

Brag-Server is an open-source REST API, written in Rust, designed to load, update, and serve data from Git repositories. It efficiently retrieves and manages repository metrics, offering JSON-formatted data for client applications. The project provides valuable insights into Git repository activities.

## Usage

Endpoints and metrics available (v0.1.0):

- `/` (root): List of all respositories registered to Brag-Server.
- `/count`: List of the sum of all commits in each respository.
  - Query string options for this endpoint:
    - `by`: if user wants the commit sums to be aggregated by a specfic time frame. This value can be either `day`, `week` or `month`.
    - `author_email`: if user wants filter the sum of commits only by a specific author email

### Requests examples

#### Request `/`:

```sh
curl brag.gubasso.xyz | jq
```

Response JSON:

```json
[
  {
    "name": "brag-server",
    "user": "cwnt-io",
    "full_name": "cwnt-io/brag-server"
  },
  {
    "name": "ripissue",
    "user": "cwnt-io",
    "full_name": "cwnt-io/ripissue"
  },
  {
    "name": "brag-server",
    "user": "gubasso",
    "full_name": "gubasso/brag-server"
  },
  {
    "name": "docs-n-notes",
    "user": "gubasso",
    "full_name": "gubasso/docs-n-notes"
  },
  {
    "name": "dotfiles",
    "user": "gubasso",
    "full_name": "gubasso/dotfiles"
  },
  {
    "name": "gubasso.xyz",
    "user": "gubasso",
    "full_name": "gubasso/gubasso.xyz"
  },
  {
    "name": "leetcode_rust.ws",
    "user": "gubasso",
    "full_name": "gubasso/leetcode_rust.ws"
  },
  {
    "name": "ripissue",
    "user": "gubasso",
    "full_name": "gubasso/ripissue"
  },
]
```

#### Request `/count`:

```sh
curl brag.gubasso.xyz/count | jq
```

Response JSON:

```json
[
  {
    "repo": {
      "name": "brag-server",
      "user": "cwnt-io",
      "full_name": "cwnt-io/brag-server"
    },
    "count": 59,
    "query": null,
    "date_agg": null
  },
  {
    "repo": {
      "name": "ripissue",
      "user": "cwnt-io",
      "full_name": "cwnt-io/ripissue"
    },
    "count": 288,
    "query": null,
    "date_agg": null
  },
    "count": 90,
    "query": null,
    "date_agg": null
  },
  {
    "repo": {
      "name": "gubasso.xyz",
      "user": "gubasso",
      "full_name": "gubasso/gubasso.xyz"
    },
    "count": 87,
    "query": null,
    "date_agg": null
  },
  {
    "repo": {
      "name": "leetcode_rust.ws",
      "user": "gubasso",
      "full_name": "gubasso/leetcode_rust.ws"
    },
    "count": 353,
    "query": null,
    "date_agg": null
  },
  ...
]
```

#### Request `/count?by=month`:

```sh
curl 'brag.gubasso.xyz/count?by=month' | jq
```

Response JSON:

```json
[
  {
    "repo": {
      "name": "ripissue",
      "user": "gubasso",
      "full_name": "gubasso/ripissue"
    },
    "count": 222,
    "query": {
      "by": "Month",
      "author_email": null,
      "repo": null
    },
    "date_agg": "2023-09-01T00:00:00Z"
  },
  ...
]
```

#### Request `/count?author_email=gubasso@cwnt.io`:

```sh
curl 'brag.gubasso.xyz/count?author_email=gubasso@cwnt.io' | jq
```

Response JSON:

```json
[
  {
    "repo": {
      "name": "brag-server",
      "user": "cwnt-io",
      "full_name": "cwnt-io/brag-server"
    },
    "count": 59,
    "query": null,
    "date_agg": null
  },
  {
    "repo": {
      "name": "ripissue",
      "user": "cwnt-io",
      "full_name": "cwnt-io/ripissue"
    },
    "count": 288,
    "query": null,
    "date_agg": null
  },
  ...
]
```

## Server Deployment ("install")

### Prerequisites

- Server must have docker and docker compose installed and working
- The `./scripts/setup_initial.sh` script sets up some of the environment dependencies automatically, but this script is written considering a **Ubuntu OS**.

### Deploy

1) Clone this repository to your server:

```sh
git clone https://github.com/cwnt-io/brag-server.git
```

2) Edit the `brag-server.toml` file with the respective `host` (github or gitlab) and user of the public repositories you want to load the metrics.

3) Edit the file `./scripts/env_vars.sh` to set the environment variables.

In projects directory, run the command:

```sh
cd brag-server
./run prod
```

The server will be up and running as a docker container.

The service will not be accessible by the `localhost` but by the `npm_net` docker network.

If you are using `Nginx Proxy Manager` as your load-balacer/reverse proxy application, you must spin-up its container within the `npm_net` network, so it will be able to access the `Brag-Server` service.
