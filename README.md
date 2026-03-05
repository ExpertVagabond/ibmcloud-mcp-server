# ibmcloud-mcp-server

[\![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[\![MCP](https://img.shields.io/badge/MCP-Compatible-blue.svg)](https://modelcontextprotocol.io)
[\![TypeScript](https://img.shields.io/badge/TypeScript-5.x-blue.svg)](https://www.typescriptlang.org)

MCP server wrapping the IBM Cloud CLI. Provides 80+ tools covering authentication, resource management, Cloud Foundry, Kubernetes, Container Registry, IAM, catalog, and billing operations.

## Tools (80+)

### Authentication (8 tools)

| Tool | Description |
|------|-------------|
| `ibmcloud_login` | Login with API key or SSO |
| `ibmcloud_logout` | Logout from IBM Cloud |
| `ibmcloud_target` | Set target region and resource group |
| `ibmcloud_api` | View or set API endpoint |
| `ibmcloud_regions` | List available regions |
| `ibmcloud_account_show` | Show current account details |
| `ibmcloud_account_list` | List accessible accounts |
| `ibmcloud_config_list` | List CLI configuration |

### Resource Management (14 tools)

`ibmcloud_resource_groups`, `ibmcloud_resource_group_create`, `ibmcloud_resource_service_instances`, `ibmcloud_resource_service_instance`, `ibmcloud_resource_service_instance_create`, `ibmcloud_resource_service_instance_delete`, `ibmcloud_resource_service_instance_update`, `ibmcloud_resource_service_keys`, `ibmcloud_resource_service_key`, `ibmcloud_resource_service_key_create`, `ibmcloud_resource_service_key_delete`, `ibmcloud_resource_search`, `ibmcloud_resource_tags`, `ibmcloud_resource_tag_attach`

### Cloud Foundry (20 tools)

`ibmcloud_cf_orgs`, `ibmcloud_cf_spaces`, `ibmcloud_cf_apps`, `ibmcloud_cf_app`, `ibmcloud_cf_push`, `ibmcloud_cf_start`, `ibmcloud_cf_stop`, `ibmcloud_cf_restart`, `ibmcloud_cf_delete`, `ibmcloud_cf_logs`, `ibmcloud_cf_env`, `ibmcloud_cf_set_env`, `ibmcloud_cf_scale`, `ibmcloud_cf_routes`, `ibmcloud_cf_services`, `ibmcloud_cf_marketplace`, `ibmcloud_cf_create_service`, `ibmcloud_cf_bind_service`, `ibmcloud_cf_unbind_service`

### Kubernetes (15 tools)

`ibmcloud_ks_clusters`, `ibmcloud_ks_cluster`, `ibmcloud_ks_cluster_config`, `ibmcloud_ks_cluster_create`, `ibmcloud_ks_cluster_delete`, `ibmcloud_ks_workers`, `ibmcloud_ks_worker_pools`, `ibmcloud_ks_worker_pool_create`, `ibmcloud_ks_worker_pool_resize`, `ibmcloud_ks_zones`, `ibmcloud_ks_versions`, `ibmcloud_ks_flavors`

### Container Registry (5 tools)

`ibmcloud_cr_namespaces`, `ibmcloud_cr_namespace_add`, `ibmcloud_cr_images`, `ibmcloud_cr_image_rm`, `ibmcloud_cr_quota`

### IAM (18 tools)

`ibmcloud_iam_users`, `ibmcloud_iam_user_invite`, `ibmcloud_iam_api_keys`, `ibmcloud_iam_api_key_create`, `ibmcloud_iam_api_key_delete`, `ibmcloud_iam_service_ids`, `ibmcloud_iam_service_id`, `ibmcloud_iam_service_id_create`, `ibmcloud_iam_service_id_delete`, `ibmcloud_iam_service_api_keys`, `ibmcloud_iam_service_api_key_create`, `ibmcloud_iam_access_groups`, `ibmcloud_iam_access_group`, `ibmcloud_iam_access_group_create`, `ibmcloud_iam_access_group_users`, `ibmcloud_iam_access_group_user_add`, `ibmcloud_iam_access_group_policies`, `ibmcloud_iam_access_group_policy_create`, `ibmcloud_iam_roles`

### Catalog and Billing (7 tools)

`ibmcloud_catalog_search`, `ibmcloud_catalog_service`, `ibmcloud_catalog_service_plans`, `ibmcloud_billing_account_usage`, `ibmcloud_billing_resource_group_usage`, `ibmcloud_billing_resource_instances_usage`, `ibmcloud_billing_org_usage`

### Plugins (4 tools)

`ibmcloud_plugin_list`, `ibmcloud_plugin_repo_plugins`, `ibmcloud_plugin_install`, `ibmcloud_version`

## Prerequisites

Install the IBM Cloud CLI:

```bash
curl -fsSL https://clis.cloud.ibm.com/install/osx | sh
```

## Install

```bash
npm install
npm run build
```

## Configuration

```json
{
  "mcpServers": {
    "ibmcloud": {
      "type": "stdio",
      "command": "node",
      "args": ["/path/to/ibmcloud-mcp/dist/index.js"]
    }
  }
}
```

## Architecture

The server wraps the `ibmcloud` CLI binary, parsing JSON output from each command into structured MCP tool responses. Authentication state is managed by the CLI itself.

## Project Structure

```
src/index.ts    # MCP server with 80+ tool definitions
src/cli.ts      # IBM Cloud CLI execution wrapper
tsconfig.json   # TypeScript configuration
dist/           # Compiled output
```

## License

[MIT](LICENSE)
