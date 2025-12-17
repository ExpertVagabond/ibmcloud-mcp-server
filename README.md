# IBM Cloud CLI MCP Server

Comprehensive MCP server wrapping the IBM Cloud CLI for Claude Code. Provides 80+ tools covering all major IBM Cloud services and operations.

## Features

- **Authentication** - Login, logout, target, regions
- **Resource Management** - Service instances, keys, resource groups
- **Cloud Foundry** - Apps, services, routes, marketplace
- **Kubernetes** - Clusters, workers, worker pools, zones
- **Container Registry** - Namespaces, images, quotas
- **IAM** - Users, API keys, service IDs, access groups
- **Catalog & Billing** - Service catalog, usage, billing

## Tool Categories

### Authentication (8 tools)
| Tool | Description |
|------|-------------|
| `ibmcloud_login` | Login with API key or SSO |
| `ibmcloud_logout` | Logout from IBM Cloud |
| `ibmcloud_target` | Set target region/resource group |
| `ibmcloud_api` | View/set API endpoint |
| `ibmcloud_regions` | List available regions |
| `ibmcloud_account_show` | Show current account |
| `ibmcloud_account_list` | List accessible accounts |
| `ibmcloud_config_list` | List CLI configuration |

### Resource Management (14 tools)
| Tool | Description |
|------|-------------|
| `ibmcloud_resource_groups` | List resource groups |
| `ibmcloud_resource_group_create` | Create resource group |
| `ibmcloud_resource_service_instances` | List service instances |
| `ibmcloud_resource_service_instance` | Get instance details |
| `ibmcloud_resource_service_instance_create` | Create instance |
| `ibmcloud_resource_service_instance_delete` | Delete instance |
| `ibmcloud_resource_service_instance_update` | Update instance |
| `ibmcloud_resource_service_keys` | List service keys |
| `ibmcloud_resource_service_key` | Get key details |
| `ibmcloud_resource_service_key_create` | Create service key |
| `ibmcloud_resource_service_key_delete` | Delete service key |
| `ibmcloud_resource_search` | Search resources |
| `ibmcloud_resource_tags` | List tags |
| `ibmcloud_resource_tag_attach` | Attach tags |

### Cloud Foundry (20 tools)
| Tool | Description |
|------|-------------|
| `ibmcloud_cf_orgs` | List organizations |
| `ibmcloud_cf_spaces` | List spaces |
| `ibmcloud_cf_apps` | List applications |
| `ibmcloud_cf_app` | Get app details |
| `ibmcloud_cf_push` | Deploy application |
| `ibmcloud_cf_start` | Start application |
| `ibmcloud_cf_stop` | Stop application |
| `ibmcloud_cf_restart` | Restart application |
| `ibmcloud_cf_delete` | Delete application |
| `ibmcloud_cf_logs` | View app logs |
| `ibmcloud_cf_env` | Show environment variables |
| `ibmcloud_cf_set_env` | Set environment variable |
| `ibmcloud_cf_scale` | Scale application |
| `ibmcloud_cf_routes` | List routes |
| `ibmcloud_cf_services` | List services |
| `ibmcloud_cf_marketplace` | List marketplace |
| `ibmcloud_cf_create_service` | Create service |
| `ibmcloud_cf_bind_service` | Bind service to app |
| `ibmcloud_cf_unbind_service` | Unbind service |

### Kubernetes (15 tools)
| Tool | Description |
|------|-------------|
| `ibmcloud_ks_clusters` | List Kubernetes clusters |
| `ibmcloud_ks_cluster` | Get cluster details |
| `ibmcloud_ks_cluster_config` | Configure kubectl |
| `ibmcloud_ks_cluster_create` | Create cluster |
| `ibmcloud_ks_cluster_delete` | Delete cluster |
| `ibmcloud_ks_workers` | List worker nodes |
| `ibmcloud_ks_worker_pools` | List worker pools |
| `ibmcloud_ks_worker_pool_create` | Create worker pool |
| `ibmcloud_ks_worker_pool_resize` | Resize worker pool |
| `ibmcloud_ks_zones` | List available zones |
| `ibmcloud_ks_versions` | List K8s versions |
| `ibmcloud_ks_flavors` | List machine flavors |

### Container Registry (5 tools)
| Tool | Description |
|------|-------------|
| `ibmcloud_cr_namespaces` | List namespaces |
| `ibmcloud_cr_namespace_add` | Create namespace |
| `ibmcloud_cr_images` | List images |
| `ibmcloud_cr_image_rm` | Remove image |
| `ibmcloud_cr_quota` | Get quota info |

### IAM (18 tools)
| Tool | Description |
|------|-------------|
| `ibmcloud_iam_users` | List users |
| `ibmcloud_iam_user_invite` | Invite user |
| `ibmcloud_iam_api_keys` | List API keys |
| `ibmcloud_iam_api_key_create` | Create API key |
| `ibmcloud_iam_api_key_delete` | Delete API key |
| `ibmcloud_iam_service_ids` | List service IDs |
| `ibmcloud_iam_service_id` | Get service ID |
| `ibmcloud_iam_service_id_create` | Create service ID |
| `ibmcloud_iam_service_id_delete` | Delete service ID |
| `ibmcloud_iam_service_api_keys` | List service API keys |
| `ibmcloud_iam_service_api_key_create` | Create service API key |
| `ibmcloud_iam_access_groups` | List access groups |
| `ibmcloud_iam_access_group` | Get access group |
| `ibmcloud_iam_access_group_create` | Create access group |
| `ibmcloud_iam_access_group_users` | List group users |
| `ibmcloud_iam_access_group_user_add` | Add user to group |
| `ibmcloud_iam_access_group_policies` | List group policies |
| `ibmcloud_iam_access_group_policy_create` | Create policy |
| `ibmcloud_iam_roles` | List IAM roles |

### Catalog & Billing (7 tools)
| Tool | Description |
|------|-------------|
| `ibmcloud_catalog_search` | Search service catalog |
| `ibmcloud_catalog_service` | Get service details |
| `ibmcloud_catalog_service_plans` | List service plans |
| `ibmcloud_billing_account_usage` | Get account usage |
| `ibmcloud_billing_resource_group_usage` | Get RG usage |
| `ibmcloud_billing_resource_instances_usage` | Get instance usage |
| `ibmcloud_billing_org_usage` | Get org usage |

### Plugins (4 tools)
| Tool | Description |
|------|-------------|
| `ibmcloud_plugin_list` | List installed plugins |
| `ibmcloud_plugin_repo_plugins` | List available plugins |
| `ibmcloud_plugin_install` | Install a plugin |
| `ibmcloud_version` | Show CLI version |

## Setup

### 1. Prerequisites

Install IBM Cloud CLI:
```bash
curl -fsSL https://clis.cloud.ibm.com/install/osx | sh
```

### 2. Install Dependencies

```bash
cd ~/mcp-servers/ibmcloud-mcp
npm install
npm run build
```

### 3. Add to Claude Code

Add to `~/.claude.json`:

```json
{
  "mcpServers": {
    "ibmcloud": {
      "type": "stdio",
      "command": "node",
      "args": ["/Users/matthewkarsten/mcp-servers/ibmcloud-mcp/dist/index.js"]
    }
  }
}
```

## Architecture

```
Claude Code (Opus 4.5)
         │
         └──▶ IBM Cloud MCP Server
                    │
                    └──▶ ibmcloud CLI
                              │
                              ├── Resource Controller
                              ├── Cloud Foundry API
                              ├── Kubernetes Service
                              ├── Container Registry
                              ├── IAM Service
                              └── Billing Service
```

## Usage Examples

```
User: List my IBM Cloud Kubernetes clusters

Claude: [Uses ibmcloud_ks_clusters tool]
Result:
- mycluster (VPC Gen2, us-south, 1.28, 3 workers) - normal
- dev-cluster (Classic, dal10, 1.27, 2 workers) - normal

User: Create a new Cloud Object Storage instance

Claude: [Uses ibmcloud_resource_service_instance_create]
Created: my-cos-instance (cloud-object-storage, lite plan)
```

## IBM Cloud Services Supported

- **Watson AI**: watsonx.ai, Watson Studio, Watson ML
- **Storage**: Cloud Object Storage, Block Storage
- **Databases**: Db2, PostgreSQL, MongoDB, Redis
- **Containers**: Kubernetes, OpenShift, Container Registry
- **Serverless**: Cloud Functions, Code Engine
- **Networking**: VPC, Load Balancers, DNS
- **Security**: Key Protect, Secrets Manager, IAM
- **Integration**: API Connect, MQ, Event Streams

## Files

```
ibmcloud-mcp/
├── src/
│   ├── index.ts    # MCP server implementation
│   └── cli.ts      # IBM Cloud CLI wrapper
├── dist/           # Compiled JavaScript
├── package.json
├── tsconfig.json
└── README.md
```

## Author

Matthew Karsten

## License

MIT
