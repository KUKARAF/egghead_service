# GitHub Token Setup

## KV keys (kv.osmosis.page)

| KV key name    | Value                        |
|----------------|------------------------------|
| `GITHUB_TOKEN` | Fine-grained Personal Access Token (see below) |
| `GITHUB_REPO`  | `layer5-5/userscripts`       |

## Token permissions

Create a **fine-grained Personal Access Token** on GitHub scoped to the `layer5-5/userscripts` repository with the following repository permissions:

| Permission      | Access         | Required for                          |
|-----------------|----------------|---------------------------------------|
| **Contents**    | Read and write | Push script `.user.js` and `.toml` files |
| **Pull requests** | Read and write | Future: AI-generated PR workflow      |

## How to create the token

1. Go to GitHub → Settings → Developer settings → Personal access tokens → Fine-grained tokens
2. Click **Generate new token**
3. Set **Repository access** → Only select repositories → `layer5-5/userscripts`
4. Under **Repository permissions** set Contents = Read and write, Pull requests = Read and write
5. Copy the generated token
6. In kv.osmosis.page, create/update the key `GITHUB_TOKEN` with the token value
