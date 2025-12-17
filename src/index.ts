#!/usr/bin/env node
import { Server } from "@modelcontextprotocol/sdk/server/index.js";
import { StdioServerTransport } from "@modelcontextprotocol/sdk/server/stdio.js";
import {
  CallToolRequestSchema,
  ListToolsRequestSchema,
  Tool,
} from "@modelcontextprotocol/sdk/types.js";
import { executeIBMCloud, formatResult } from "./cli.js";

// Authentication Tools
const authTools: Tool[] = [
  {
    name: "ibmcloud_login",
    description: "Login to IBM Cloud with API key or SSO",
    inputSchema: {
      type: "object",
      properties: {
        apikey: { type: "string", description: "IBM Cloud API key" },
        sso: { type: "boolean", description: "Use SSO for login" },
        region: { type: "string", description: "Target region (e.g., us-south)" },
      },
    },
  },
  {
    name: "ibmcloud_logout",
    description: "Logout from IBM Cloud",
    inputSchema: { type: "object", properties: {} },
  },
  {
    name: "ibmcloud_target",
    description: "View or set target region, resource group, org, and space",
    inputSchema: {
      type: "object",
      properties: {
        region: { type: "string", description: "Target region" },
        resource_group: { type: "string", description: "Target resource group" },
        org: { type: "string", description: "Target Cloud Foundry org" },
        space: { type: "string", description: "Target Cloud Foundry space" },
      },
    },
  },
  {
    name: "ibmcloud_api",
    description: "View or set IBM Cloud API endpoint",
    inputSchema: {
      type: "object",
      properties: {
        endpoint: { type: "string", description: "API endpoint URL" },
      },
    },
  },
  {
    name: "ibmcloud_regions",
    description: "List available IBM Cloud regions",
    inputSchema: { type: "object", properties: {} },
  },
  {
    name: "ibmcloud_account_show",
    description: "Show current account information",
    inputSchema: { type: "object", properties: {} },
  },
  {
    name: "ibmcloud_account_list",
    description: "List all accessible accounts",
    inputSchema: { type: "object", properties: {} },
  },
  {
    name: "ibmcloud_config_list",
    description: "List CLI configuration",
    inputSchema: { type: "object", properties: {} },
  },
];

// Resource Management Tools
const resourceTools: Tool[] = [
  {
    name: "ibmcloud_resource_groups",
    description: "List resource groups",
    inputSchema: { type: "object", properties: {} },
  },
  {
    name: "ibmcloud_resource_group_create",
    description: "Create a resource group",
    inputSchema: {
      type: "object",
      properties: {
        name: { type: "string", description: "Resource group name" },
      },
      required: ["name"],
    },
  },
  {
    name: "ibmcloud_resource_service_instances",
    description: "List service instances",
    inputSchema: {
      type: "object",
      properties: {
        service_name: { type: "string", description: "Filter by service name" },
        resource_group: { type: "string", description: "Filter by resource group" },
      },
    },
  },
  {
    name: "ibmcloud_resource_service_instance",
    description: "Get details of a service instance",
    inputSchema: {
      type: "object",
      properties: {
        name: { type: "string", description: "Service instance name or ID" },
      },
      required: ["name"],
    },
  },
  {
    name: "ibmcloud_resource_service_instance_create",
    description: "Create a service instance",
    inputSchema: {
      type: "object",
      properties: {
        name: { type: "string", description: "Instance name" },
        service: { type: "string", description: "Service name (e.g., cloud-object-storage)" },
        plan: { type: "string", description: "Service plan (e.g., standard, lite)" },
        location: { type: "string", description: "Location (e.g., global, us-south)" },
        resource_group: { type: "string", description: "Resource group name" },
        parameters: { type: "string", description: "Service-specific parameters as JSON" },
      },
      required: ["name", "service", "plan"],
    },
  },
  {
    name: "ibmcloud_resource_service_instance_delete",
    description: "Delete a service instance",
    inputSchema: {
      type: "object",
      properties: {
        name: { type: "string", description: "Service instance name or ID" },
        force: { type: "boolean", description: "Force deletion without confirmation" },
      },
      required: ["name"],
    },
  },
  {
    name: "ibmcloud_resource_service_instance_update",
    description: "Update a service instance",
    inputSchema: {
      type: "object",
      properties: {
        name: { type: "string", description: "Service instance name or ID" },
        new_name: { type: "string", description: "New name for the instance" },
        plan: { type: "string", description: "New service plan" },
        parameters: { type: "string", description: "Service-specific parameters as JSON" },
      },
      required: ["name"],
    },
  },
  {
    name: "ibmcloud_resource_service_keys",
    description: "List service keys/credentials",
    inputSchema: {
      type: "object",
      properties: {
        instance: { type: "string", description: "Service instance name" },
      },
    },
  },
  {
    name: "ibmcloud_resource_service_key",
    description: "Get details of a service key",
    inputSchema: {
      type: "object",
      properties: {
        name: { type: "string", description: "Service key name" },
      },
      required: ["name"],
    },
  },
  {
    name: "ibmcloud_resource_service_key_create",
    description: "Create a service key/credential",
    inputSchema: {
      type: "object",
      properties: {
        name: { type: "string", description: "Key name" },
        instance: { type: "string", description: "Service instance name" },
        role: { type: "string", description: "IAM role (e.g., Writer, Reader, Manager)" },
        parameters: { type: "string", description: "Service-specific parameters as JSON" },
      },
      required: ["name", "instance"],
    },
  },
  {
    name: "ibmcloud_resource_service_key_delete",
    description: "Delete a service key",
    inputSchema: {
      type: "object",
      properties: {
        name: { type: "string", description: "Service key name" },
        force: { type: "boolean", description: "Force deletion without confirmation" },
      },
      required: ["name"],
    },
  },
  {
    name: "ibmcloud_resource_search",
    description: "Search for resources",
    inputSchema: {
      type: "object",
      properties: {
        query: { type: "string", description: "Search query (Lucene syntax)" },
      },
      required: ["query"],
    },
  },
  {
    name: "ibmcloud_resource_tags",
    description: "List tags",
    inputSchema: {
      type: "object",
      properties: {
        resource_id: { type: "string", description: "Filter by resource CRN" },
      },
    },
  },
  {
    name: "ibmcloud_resource_tag_attach",
    description: "Attach tags to a resource",
    inputSchema: {
      type: "object",
      properties: {
        resource_id: { type: "string", description: "Resource CRN" },
        tags: { type: "string", description: "Comma-separated list of tags" },
      },
      required: ["resource_id", "tags"],
    },
  },
];

// Cloud Foundry Tools
const cfTools: Tool[] = [
  {
    name: "ibmcloud_cf_orgs",
    description: "List Cloud Foundry organizations",
    inputSchema: { type: "object", properties: {} },
  },
  {
    name: "ibmcloud_cf_spaces",
    description: "List Cloud Foundry spaces",
    inputSchema: {
      type: "object",
      properties: {
        org: { type: "string", description: "Organization name" },
      },
    },
  },
  {
    name: "ibmcloud_cf_apps",
    description: "List Cloud Foundry applications",
    inputSchema: { type: "object", properties: {} },
  },
  {
    name: "ibmcloud_cf_app",
    description: "Get Cloud Foundry application details",
    inputSchema: {
      type: "object",
      properties: {
        name: { type: "string", description: "Application name" },
      },
      required: ["name"],
    },
  },
  {
    name: "ibmcloud_cf_push",
    description: "Deploy a Cloud Foundry application",
    inputSchema: {
      type: "object",
      properties: {
        name: { type: "string", description: "Application name" },
        path: { type: "string", description: "Path to application" },
        manifest: { type: "string", description: "Path to manifest file" },
        memory: { type: "string", description: "Memory limit (e.g., 256M, 1G)" },
        instances: { type: "number", description: "Number of instances" },
        buildpack: { type: "string", description: "Buildpack name or URL" },
        docker_image: { type: "string", description: "Docker image" },
      },
    },
  },
  {
    name: "ibmcloud_cf_start",
    description: "Start a Cloud Foundry application",
    inputSchema: {
      type: "object",
      properties: {
        name: { type: "string", description: "Application name" },
      },
      required: ["name"],
    },
  },
  {
    name: "ibmcloud_cf_stop",
    description: "Stop a Cloud Foundry application",
    inputSchema: {
      type: "object",
      properties: {
        name: { type: "string", description: "Application name" },
      },
      required: ["name"],
    },
  },
  {
    name: "ibmcloud_cf_restart",
    description: "Restart a Cloud Foundry application",
    inputSchema: {
      type: "object",
      properties: {
        name: { type: "string", description: "Application name" },
      },
      required: ["name"],
    },
  },
  {
    name: "ibmcloud_cf_delete",
    description: "Delete a Cloud Foundry application",
    inputSchema: {
      type: "object",
      properties: {
        name: { type: "string", description: "Application name" },
        force: { type: "boolean", description: "Force deletion without confirmation" },
        delete_routes: { type: "boolean", description: "Also delete mapped routes" },
      },
      required: ["name"],
    },
  },
  {
    name: "ibmcloud_cf_logs",
    description: "View recent logs for an application",
    inputSchema: {
      type: "object",
      properties: {
        name: { type: "string", description: "Application name" },
        recent: { type: "boolean", description: "Show recent logs (default: true)" },
      },
      required: ["name"],
    },
  },
  {
    name: "ibmcloud_cf_env",
    description: "Show environment variables for an application",
    inputSchema: {
      type: "object",
      properties: {
        name: { type: "string", description: "Application name" },
      },
      required: ["name"],
    },
  },
  {
    name: "ibmcloud_cf_set_env",
    description: "Set an environment variable for an application",
    inputSchema: {
      type: "object",
      properties: {
        name: { type: "string", description: "Application name" },
        var_name: { type: "string", description: "Environment variable name" },
        var_value: { type: "string", description: "Environment variable value" },
      },
      required: ["name", "var_name", "var_value"],
    },
  },
  {
    name: "ibmcloud_cf_scale",
    description: "Scale a Cloud Foundry application",
    inputSchema: {
      type: "object",
      properties: {
        name: { type: "string", description: "Application name" },
        instances: { type: "number", description: "Number of instances" },
        memory: { type: "string", description: "Memory limit (e.g., 512M)" },
        disk: { type: "string", description: "Disk limit (e.g., 1G)" },
      },
      required: ["name"],
    },
  },
  {
    name: "ibmcloud_cf_routes",
    description: "List Cloud Foundry routes",
    inputSchema: { type: "object", properties: {} },
  },
  {
    name: "ibmcloud_cf_services",
    description: "List Cloud Foundry services",
    inputSchema: { type: "object", properties: {} },
  },
  {
    name: "ibmcloud_cf_marketplace",
    description: "List marketplace services",
    inputSchema: {
      type: "object",
      properties: {
        service: { type: "string", description: "Show plans for a specific service" },
      },
    },
  },
  {
    name: "ibmcloud_cf_create_service",
    description: "Create a Cloud Foundry service instance",
    inputSchema: {
      type: "object",
      properties: {
        service: { type: "string", description: "Service offering name" },
        plan: { type: "string", description: "Service plan" },
        instance_name: { type: "string", description: "Service instance name" },
        parameters: { type: "string", description: "JSON parameters" },
      },
      required: ["service", "plan", "instance_name"],
    },
  },
  {
    name: "ibmcloud_cf_bind_service",
    description: "Bind a service to an application",
    inputSchema: {
      type: "object",
      properties: {
        app_name: { type: "string", description: "Application name" },
        service_instance: { type: "string", description: "Service instance name" },
        parameters: { type: "string", description: "JSON binding parameters" },
      },
      required: ["app_name", "service_instance"],
    },
  },
  {
    name: "ibmcloud_cf_unbind_service",
    description: "Unbind a service from an application",
    inputSchema: {
      type: "object",
      properties: {
        app_name: { type: "string", description: "Application name" },
        service_instance: { type: "string", description: "Service instance name" },
      },
      required: ["app_name", "service_instance"],
    },
  },
];

// Kubernetes & Container Registry Tools
const ksTools: Tool[] = [
  {
    name: "ibmcloud_ks_clusters",
    description: "List Kubernetes clusters",
    inputSchema: {
      type: "object",
      properties: {
        provider: { type: "string", description: "Filter by provider (vpc-gen2, classic)" },
      },
    },
  },
  {
    name: "ibmcloud_ks_cluster",
    description: "Get cluster details",
    inputSchema: {
      type: "object",
      properties: {
        cluster: { type: "string", description: "Cluster name or ID" },
      },
      required: ["cluster"],
    },
  },
  {
    name: "ibmcloud_ks_cluster_config",
    description: "Configure kubectl for a cluster",
    inputSchema: {
      type: "object",
      properties: {
        cluster: { type: "string", description: "Cluster name or ID" },
        admin: { type: "boolean", description: "Download admin certificates" },
      },
      required: ["cluster"],
    },
  },
  {
    name: "ibmcloud_ks_cluster_create",
    description: "Create a Kubernetes cluster",
    inputSchema: {
      type: "object",
      properties: {
        name: { type: "string", description: "Cluster name" },
        zone: { type: "string", description: "Zone (e.g., dal10)" },
        flavor: { type: "string", description: "Worker node flavor" },
        workers: { type: "number", description: "Number of workers" },
        version: { type: "string", description: "Kubernetes version" },
        vpc_id: { type: "string", description: "VPC ID for VPC clusters" },
        subnet_id: { type: "string", description: "Subnet ID for VPC clusters" },
      },
      required: ["name", "zone"],
    },
  },
  {
    name: "ibmcloud_ks_cluster_delete",
    description: "Delete a Kubernetes cluster",
    inputSchema: {
      type: "object",
      properties: {
        cluster: { type: "string", description: "Cluster name or ID" },
        force: { type: "boolean", description: "Force deletion" },
      },
      required: ["cluster"],
    },
  },
  {
    name: "ibmcloud_ks_workers",
    description: "List worker nodes in a cluster",
    inputSchema: {
      type: "object",
      properties: {
        cluster: { type: "string", description: "Cluster name or ID" },
      },
      required: ["cluster"],
    },
  },
  {
    name: "ibmcloud_ks_worker_pools",
    description: "List worker pools in a cluster",
    inputSchema: {
      type: "object",
      properties: {
        cluster: { type: "string", description: "Cluster name or ID" },
      },
      required: ["cluster"],
    },
  },
  {
    name: "ibmcloud_ks_worker_pool_create",
    description: "Create a worker pool",
    inputSchema: {
      type: "object",
      properties: {
        cluster: { type: "string", description: "Cluster name or ID" },
        name: { type: "string", description: "Worker pool name" },
        flavor: { type: "string", description: "Worker node flavor" },
        size_per_zone: { type: "number", description: "Workers per zone" },
      },
      required: ["cluster", "name", "flavor"],
    },
  },
  {
    name: "ibmcloud_ks_worker_pool_resize",
    description: "Resize a worker pool",
    inputSchema: {
      type: "object",
      properties: {
        cluster: { type: "string", description: "Cluster name or ID" },
        pool: { type: "string", description: "Worker pool name" },
        size_per_zone: { type: "number", description: "New workers per zone" },
      },
      required: ["cluster", "pool", "size_per_zone"],
    },
  },
  {
    name: "ibmcloud_ks_zones",
    description: "List available zones",
    inputSchema: {
      type: "object",
      properties: {
        provider: { type: "string", description: "Filter by provider" },
        location: { type: "string", description: "Filter by location" },
      },
    },
  },
  {
    name: "ibmcloud_ks_versions",
    description: "List available Kubernetes versions",
    inputSchema: { type: "object", properties: {} },
  },
  {
    name: "ibmcloud_ks_flavors",
    description: "List available machine types/flavors",
    inputSchema: {
      type: "object",
      properties: {
        zone: { type: "string", description: "Zone to list flavors for" },
        provider: { type: "string", description: "Provider (vpc-gen2, classic)" },
      },
      required: ["zone"],
    },
  },
  {
    name: "ibmcloud_cr_namespaces",
    description: "List Container Registry namespaces",
    inputSchema: { type: "object", properties: {} },
  },
  {
    name: "ibmcloud_cr_namespace_add",
    description: "Create a Container Registry namespace",
    inputSchema: {
      type: "object",
      properties: {
        name: { type: "string", description: "Namespace name" },
        resource_group: { type: "string", description: "Resource group" },
      },
      required: ["name"],
    },
  },
  {
    name: "ibmcloud_cr_images",
    description: "List container images",
    inputSchema: {
      type: "object",
      properties: {
        repository: { type: "string", description: "Filter by repository" },
        include_ibm: { type: "boolean", description: "Include IBM images" },
      },
    },
  },
  {
    name: "ibmcloud_cr_image_rm",
    description: "Remove a container image",
    inputSchema: {
      type: "object",
      properties: {
        image: { type: "string", description: "Image name with tag" },
        force: { type: "boolean", description: "Force deletion" },
      },
      required: ["image"],
    },
  },
  {
    name: "ibmcloud_cr_quota",
    description: "Get Container Registry quota information",
    inputSchema: { type: "object", properties: {} },
  },
];

// IAM Tools
const iamTools: Tool[] = [
  {
    name: "ibmcloud_iam_users",
    description: "List users in the account",
    inputSchema: { type: "object", properties: {} },
  },
  {
    name: "ibmcloud_iam_user_invite",
    description: "Invite a user to the account",
    inputSchema: {
      type: "object",
      properties: {
        email: { type: "string", description: "User email address" },
        access_groups: { type: "string", description: "Comma-separated access group names" },
      },
      required: ["email"],
    },
  },
  {
    name: "ibmcloud_iam_api_keys",
    description: "List API keys",
    inputSchema: { type: "object", properties: {} },
  },
  {
    name: "ibmcloud_iam_api_key_create",
    description: "Create an API key",
    inputSchema: {
      type: "object",
      properties: {
        name: { type: "string", description: "API key name" },
        description: { type: "string", description: "API key description" },
        file: { type: "string", description: "File path to save the key" },
      },
      required: ["name"],
    },
  },
  {
    name: "ibmcloud_iam_api_key_delete",
    description: "Delete an API key",
    inputSchema: {
      type: "object",
      properties: {
        name: { type: "string", description: "API key name or ID" },
        force: { type: "boolean", description: "Force deletion" },
      },
      required: ["name"],
    },
  },
  {
    name: "ibmcloud_iam_service_ids",
    description: "List service IDs",
    inputSchema: { type: "object", properties: {} },
  },
  {
    name: "ibmcloud_iam_service_id",
    description: "Get service ID details",
    inputSchema: {
      type: "object",
      properties: {
        name: { type: "string", description: "Service ID name or UUID" },
      },
      required: ["name"],
    },
  },
  {
    name: "ibmcloud_iam_service_id_create",
    description: "Create a service ID",
    inputSchema: {
      type: "object",
      properties: {
        name: { type: "string", description: "Service ID name" },
        description: { type: "string", description: "Description" },
      },
      required: ["name"],
    },
  },
  {
    name: "ibmcloud_iam_service_id_delete",
    description: "Delete a service ID",
    inputSchema: {
      type: "object",
      properties: {
        name: { type: "string", description: "Service ID name or UUID" },
        force: { type: "boolean", description: "Force deletion" },
      },
      required: ["name"],
    },
  },
  {
    name: "ibmcloud_iam_service_api_keys",
    description: "List API keys for a service ID",
    inputSchema: {
      type: "object",
      properties: {
        service_id: { type: "string", description: "Service ID name or UUID" },
      },
      required: ["service_id"],
    },
  },
  {
    name: "ibmcloud_iam_service_api_key_create",
    description: "Create an API key for a service ID",
    inputSchema: {
      type: "object",
      properties: {
        name: { type: "string", description: "API key name" },
        service_id: { type: "string", description: "Service ID name or UUID" },
        description: { type: "string", description: "Description" },
        file: { type: "string", description: "File path to save the key" },
      },
      required: ["name", "service_id"],
    },
  },
  {
    name: "ibmcloud_iam_access_groups",
    description: "List access groups",
    inputSchema: { type: "object", properties: {} },
  },
  {
    name: "ibmcloud_iam_access_group",
    description: "Get access group details",
    inputSchema: {
      type: "object",
      properties: {
        name: { type: "string", description: "Access group name" },
      },
      required: ["name"],
    },
  },
  {
    name: "ibmcloud_iam_access_group_create",
    description: "Create an access group",
    inputSchema: {
      type: "object",
      properties: {
        name: { type: "string", description: "Access group name" },
        description: { type: "string", description: "Description" },
      },
      required: ["name"],
    },
  },
  {
    name: "ibmcloud_iam_access_group_users",
    description: "List users in an access group",
    inputSchema: {
      type: "object",
      properties: {
        group: { type: "string", description: "Access group name" },
      },
      required: ["group"],
    },
  },
  {
    name: "ibmcloud_iam_access_group_user_add",
    description: "Add users to an access group",
    inputSchema: {
      type: "object",
      properties: {
        group: { type: "string", description: "Access group name" },
        users: { type: "string", description: "Comma-separated user IDs" },
      },
      required: ["group", "users"],
    },
  },
  {
    name: "ibmcloud_iam_access_group_policies",
    description: "List policies for an access group",
    inputSchema: {
      type: "object",
      properties: {
        group: { type: "string", description: "Access group name" },
      },
      required: ["group"],
    },
  },
  {
    name: "ibmcloud_iam_access_group_policy_create",
    description: "Create a policy for an access group",
    inputSchema: {
      type: "object",
      properties: {
        group: { type: "string", description: "Access group name" },
        roles: { type: "string", description: "Comma-separated roles" },
        service_name: { type: "string", description: "Service name" },
        resource_group: { type: "string", description: "Resource group name" },
        resource_type: { type: "string", description: "Resource type" },
        resource: { type: "string", description: "Resource name" },
      },
      required: ["group", "roles"],
    },
  },
  {
    name: "ibmcloud_iam_roles",
    description: "List IAM roles",
    inputSchema: {
      type: "object",
      properties: {
        service: { type: "string", description: "Filter by service name" },
      },
    },
  },
];

// Catalog & Billing Tools
const catalogBillingTools: Tool[] = [
  {
    name: "ibmcloud_catalog_search",
    description: "Search the service catalog",
    inputSchema: {
      type: "object",
      properties: {
        query: { type: "string", description: "Search query" },
      },
      required: ["query"],
    },
  },
  {
    name: "ibmcloud_catalog_service",
    description: "Get details about a catalog service",
    inputSchema: {
      type: "object",
      properties: {
        service: { type: "string", description: "Service name" },
      },
      required: ["service"],
    },
  },
  {
    name: "ibmcloud_catalog_service_plans",
    description: "List plans for a catalog service",
    inputSchema: {
      type: "object",
      properties: {
        service: { type: "string", description: "Service name" },
      },
      required: ["service"],
    },
  },
  {
    name: "ibmcloud_billing_account_usage",
    description: "Get account usage for a billing period",
    inputSchema: {
      type: "object",
      properties: {
        month: { type: "string", description: "Month (YYYY-MM)" },
      },
    },
  },
  {
    name: "ibmcloud_billing_resource_group_usage",
    description: "Get resource group usage",
    inputSchema: {
      type: "object",
      properties: {
        resource_group: { type: "string", description: "Resource group name" },
        month: { type: "string", description: "Month (YYYY-MM)" },
      },
      required: ["resource_group"],
    },
  },
  {
    name: "ibmcloud_billing_resource_instances_usage",
    description: "Get usage for resource instances",
    inputSchema: {
      type: "object",
      properties: {
        month: { type: "string", description: "Month (YYYY-MM)" },
        resource_group: { type: "string", description: "Filter by resource group" },
      },
    },
  },
  {
    name: "ibmcloud_billing_org_usage",
    description: "Get Cloud Foundry org usage",
    inputSchema: {
      type: "object",
      properties: {
        org: { type: "string", description: "Organization name" },
        month: { type: "string", description: "Month (YYYY-MM)" },
      },
      required: ["org"],
    },
  },
  {
    name: "ibmcloud_plugin_list",
    description: "List installed CLI plugins",
    inputSchema: { type: "object", properties: {} },
  },
  {
    name: "ibmcloud_plugin_repo_plugins",
    description: "List available plugins from repository",
    inputSchema: { type: "object", properties: {} },
  },
  {
    name: "ibmcloud_plugin_install",
    description: "Install a CLI plugin",
    inputSchema: {
      type: "object",
      properties: {
        plugin: { type: "string", description: "Plugin name" },
        force: { type: "boolean", description: "Force reinstall if exists" },
      },
      required: ["plugin"],
    },
  },
  {
    name: "ibmcloud_version",
    description: "Show IBM Cloud CLI version",
    inputSchema: { type: "object", properties: {} },
  },
];

// All tools combined
const allTools = [
  ...authTools,
  ...resourceTools,
  ...cfTools,
  ...ksTools,
  ...iamTools,
  ...catalogBillingTools,
];

// Tool handlers
async function handleAuthTool(name: string, args: Record<string, unknown>): Promise<string> {
  switch (name) {
    case "ibmcloud_login": {
      const cmdArgs = ["login"];
      if (args.apikey) cmdArgs.push("--apikey", args.apikey as string);
      if (args.sso) cmdArgs.push("--sso");
      if (args.region) cmdArgs.push("-r", args.region as string);
      return formatResult(await executeIBMCloud(cmdArgs));
    }
    case "ibmcloud_logout":
      return formatResult(await executeIBMCloud(["logout"]));
    case "ibmcloud_target": {
      const cmdArgs = ["target"];
      if (args.region) cmdArgs.push("-r", args.region as string);
      if (args.resource_group) cmdArgs.push("-g", args.resource_group as string);
      if (args.org) cmdArgs.push("-o", args.org as string);
      if (args.space) cmdArgs.push("-s", args.space as string);
      return formatResult(await executeIBMCloud(cmdArgs));
    }
    case "ibmcloud_api": {
      const cmdArgs = ["api"];
      if (args.endpoint) cmdArgs.push(args.endpoint as string);
      return formatResult(await executeIBMCloud(cmdArgs));
    }
    case "ibmcloud_regions":
      return formatResult(await executeIBMCloud(["regions"]));
    case "ibmcloud_account_show":
      return formatResult(await executeIBMCloud(["account", "show"]));
    case "ibmcloud_account_list":
      return formatResult(await executeIBMCloud(["account", "list"]));
    case "ibmcloud_config_list":
      return formatResult(await executeIBMCloud(["config", "list"]));
    default:
      throw new Error(`Unknown auth tool: ${name}`);
  }
}

async function handleResourceTool(name: string, args: Record<string, unknown>): Promise<string> {
  switch (name) {
    case "ibmcloud_resource_groups":
      return formatResult(await executeIBMCloud(["resource", "groups"]));
    case "ibmcloud_resource_group_create":
      return formatResult(await executeIBMCloud(["resource", "group-create", args.name as string]));
    case "ibmcloud_resource_service_instances": {
      const cmdArgs = ["resource", "service-instances"];
      if (args.service_name) cmdArgs.push("--service-name", args.service_name as string);
      if (args.resource_group) cmdArgs.push("-g", args.resource_group as string);
      return formatResult(await executeIBMCloud(cmdArgs));
    }
    case "ibmcloud_resource_service_instance":
      return formatResult(await executeIBMCloud(["resource", "service-instance", args.name as string]));
    case "ibmcloud_resource_service_instance_create": {
      const cmdArgs = ["resource", "service-instance-create", args.name as string, args.service as string, args.plan as string];
      if (args.location) cmdArgs.push("-l", args.location as string);
      if (args.resource_group) cmdArgs.push("-g", args.resource_group as string);
      if (args.parameters) cmdArgs.push("-p", args.parameters as string);
      return formatResult(await executeIBMCloud(cmdArgs));
    }
    case "ibmcloud_resource_service_instance_delete": {
      const cmdArgs = ["resource", "service-instance-delete", args.name as string];
      if (args.force) cmdArgs.push("-f");
      return formatResult(await executeIBMCloud(cmdArgs));
    }
    case "ibmcloud_resource_service_instance_update": {
      const cmdArgs = ["resource", "service-instance-update", args.name as string];
      if (args.new_name) cmdArgs.push("-n", args.new_name as string);
      if (args.plan) cmdArgs.push("--service-plan-id", args.plan as string);
      if (args.parameters) cmdArgs.push("-p", args.parameters as string);
      return formatResult(await executeIBMCloud(cmdArgs));
    }
    case "ibmcloud_resource_service_keys": {
      const cmdArgs = ["resource", "service-keys"];
      if (args.instance) cmdArgs.push("--instance-name", args.instance as string);
      return formatResult(await executeIBMCloud(cmdArgs));
    }
    case "ibmcloud_resource_service_key":
      return formatResult(await executeIBMCloud(["resource", "service-key", args.name as string]));
    case "ibmcloud_resource_service_key_create": {
      const cmdArgs = ["resource", "service-key-create", args.name as string, "--instance-name", args.instance as string];
      if (args.role) cmdArgs.push("--service-endpoint", args.role as string);
      if (args.parameters) cmdArgs.push("-p", args.parameters as string);
      return formatResult(await executeIBMCloud(cmdArgs));
    }
    case "ibmcloud_resource_service_key_delete": {
      const cmdArgs = ["resource", "service-key-delete", args.name as string];
      if (args.force) cmdArgs.push("-f");
      return formatResult(await executeIBMCloud(cmdArgs));
    }
    case "ibmcloud_resource_search":
      return formatResult(await executeIBMCloud(["resource", "search", args.query as string]));
    case "ibmcloud_resource_tags": {
      const cmdArgs = ["resource", "tags"];
      if (args.resource_id) cmdArgs.push("--tag-type", "user");
      return formatResult(await executeIBMCloud(cmdArgs));
    }
    case "ibmcloud_resource_tag_attach":
      return formatResult(await executeIBMCloud(["resource", "tag-attach", "--tag-names", args.tags as string, "--resource-id", args.resource_id as string]));
    default:
      throw new Error(`Unknown resource tool: ${name}`);
  }
}

async function handleCFTool(name: string, args: Record<string, unknown>): Promise<string> {
  switch (name) {
    case "ibmcloud_cf_orgs":
      return formatResult(await executeIBMCloud(["cf", "orgs"]));
    case "ibmcloud_cf_spaces": {
      const cmdArgs = ["cf", "spaces"];
      if (args.org) cmdArgs.push("-o", args.org as string);
      return formatResult(await executeIBMCloud(cmdArgs));
    }
    case "ibmcloud_cf_apps":
      return formatResult(await executeIBMCloud(["cf", "apps"]));
    case "ibmcloud_cf_app":
      return formatResult(await executeIBMCloud(["cf", "app", args.name as string]));
    case "ibmcloud_cf_push": {
      const cmdArgs = ["cf", "push"];
      if (args.name) cmdArgs.push(args.name as string);
      if (args.path) cmdArgs.push("-p", args.path as string);
      if (args.manifest) cmdArgs.push("-f", args.manifest as string);
      if (args.memory) cmdArgs.push("-m", args.memory as string);
      if (args.instances) cmdArgs.push("-i", String(args.instances));
      if (args.buildpack) cmdArgs.push("-b", args.buildpack as string);
      if (args.docker_image) cmdArgs.push("--docker-image", args.docker_image as string);
      return formatResult(await executeIBMCloud(cmdArgs));
    }
    case "ibmcloud_cf_start":
      return formatResult(await executeIBMCloud(["cf", "start", args.name as string]));
    case "ibmcloud_cf_stop":
      return formatResult(await executeIBMCloud(["cf", "stop", args.name as string]));
    case "ibmcloud_cf_restart":
      return formatResult(await executeIBMCloud(["cf", "restart", args.name as string]));
    case "ibmcloud_cf_delete": {
      const cmdArgs = ["cf", "delete", args.name as string];
      if (args.force) cmdArgs.push("-f");
      if (args.delete_routes) cmdArgs.push("-r");
      return formatResult(await executeIBMCloud(cmdArgs));
    }
    case "ibmcloud_cf_logs": {
      const cmdArgs = ["cf", "logs", args.name as string];
      if (args.recent !== false) cmdArgs.push("--recent");
      return formatResult(await executeIBMCloud(cmdArgs));
    }
    case "ibmcloud_cf_env":
      return formatResult(await executeIBMCloud(["cf", "env", args.name as string]));
    case "ibmcloud_cf_set_env":
      return formatResult(await executeIBMCloud(["cf", "set-env", args.name as string, args.var_name as string, args.var_value as string]));
    case "ibmcloud_cf_scale": {
      const cmdArgs = ["cf", "scale", args.name as string];
      if (args.instances) cmdArgs.push("-i", String(args.instances));
      if (args.memory) cmdArgs.push("-m", args.memory as string);
      if (args.disk) cmdArgs.push("-k", args.disk as string);
      cmdArgs.push("-f");
      return formatResult(await executeIBMCloud(cmdArgs));
    }
    case "ibmcloud_cf_routes":
      return formatResult(await executeIBMCloud(["cf", "routes"]));
    case "ibmcloud_cf_services":
      return formatResult(await executeIBMCloud(["cf", "services"]));
    case "ibmcloud_cf_marketplace": {
      const cmdArgs = ["cf", "marketplace"];
      if (args.service) cmdArgs.push("-e", args.service as string);
      return formatResult(await executeIBMCloud(cmdArgs));
    }
    case "ibmcloud_cf_create_service": {
      const cmdArgs = ["cf", "create-service", args.service as string, args.plan as string, args.instance_name as string];
      if (args.parameters) cmdArgs.push("-c", args.parameters as string);
      return formatResult(await executeIBMCloud(cmdArgs));
    }
    case "ibmcloud_cf_bind_service": {
      const cmdArgs = ["cf", "bind-service", args.app_name as string, args.service_instance as string];
      if (args.parameters) cmdArgs.push("-c", args.parameters as string);
      return formatResult(await executeIBMCloud(cmdArgs));
    }
    case "ibmcloud_cf_unbind_service":
      return formatResult(await executeIBMCloud(["cf", "unbind-service", args.app_name as string, args.service_instance as string]));
    default:
      throw new Error(`Unknown CF tool: ${name}`);
  }
}

async function handleKSTool(name: string, args: Record<string, unknown>): Promise<string> {
  switch (name) {
    case "ibmcloud_ks_clusters": {
      const cmdArgs = ["ks", "clusters"];
      if (args.provider) cmdArgs.push("--provider", args.provider as string);
      return formatResult(await executeIBMCloud(cmdArgs));
    }
    case "ibmcloud_ks_cluster":
      return formatResult(await executeIBMCloud(["ks", "cluster", "get", "--cluster", args.cluster as string]));
    case "ibmcloud_ks_cluster_config": {
      const cmdArgs = ["ks", "cluster", "config", "--cluster", args.cluster as string];
      if (args.admin) cmdArgs.push("--admin");
      return formatResult(await executeIBMCloud(cmdArgs));
    }
    case "ibmcloud_ks_cluster_create": {
      const cmdArgs = ["ks", "cluster", "create", "classic", "--name", args.name as string, "--zone", args.zone as string];
      if (args.flavor) cmdArgs.push("--flavor", args.flavor as string);
      if (args.workers) cmdArgs.push("--workers", String(args.workers));
      if (args.version) cmdArgs.push("--version", args.version as string);
      if (args.vpc_id) {
        cmdArgs[4] = "vpc-gen2";
        cmdArgs.push("--vpc-id", args.vpc_id as string);
      }
      if (args.subnet_id) cmdArgs.push("--subnet-id", args.subnet_id as string);
      return formatResult(await executeIBMCloud(cmdArgs));
    }
    case "ibmcloud_ks_cluster_delete": {
      const cmdArgs = ["ks", "cluster", "rm", "--cluster", args.cluster as string];
      if (args.force) cmdArgs.push("-f");
      return formatResult(await executeIBMCloud(cmdArgs));
    }
    case "ibmcloud_ks_workers":
      return formatResult(await executeIBMCloud(["ks", "workers", "--cluster", args.cluster as string]));
    case "ibmcloud_ks_worker_pools":
      return formatResult(await executeIBMCloud(["ks", "worker-pools", "--cluster", args.cluster as string]));
    case "ibmcloud_ks_worker_pool_create": {
      const cmdArgs = ["ks", "worker-pool", "create", "classic", "--cluster", args.cluster as string, "--name", args.name as string, "--flavor", args.flavor as string];
      if (args.size_per_zone) cmdArgs.push("--size-per-zone", String(args.size_per_zone));
      return formatResult(await executeIBMCloud(cmdArgs));
    }
    case "ibmcloud_ks_worker_pool_resize":
      return formatResult(await executeIBMCloud(["ks", "worker-pool", "resize", "--cluster", args.cluster as string, "--worker-pool", args.pool as string, "--size-per-zone", String(args.size_per_zone)]));
    case "ibmcloud_ks_zones": {
      const cmdArgs = ["ks", "zones"];
      if (args.provider) cmdArgs.push("--provider", args.provider as string);
      if (args.location) cmdArgs.push("--location", args.location as string);
      return formatResult(await executeIBMCloud(cmdArgs));
    }
    case "ibmcloud_ks_versions":
      return formatResult(await executeIBMCloud(["ks", "versions"]));
    case "ibmcloud_ks_flavors": {
      const cmdArgs = ["ks", "flavors", "--zone", args.zone as string];
      if (args.provider) cmdArgs.push("--provider", args.provider as string);
      return formatResult(await executeIBMCloud(cmdArgs));
    }
    case "ibmcloud_cr_namespaces":
      return formatResult(await executeIBMCloud(["cr", "namespaces"]));
    case "ibmcloud_cr_namespace_add": {
      const cmdArgs = ["cr", "namespace-add", args.name as string];
      if (args.resource_group) cmdArgs.push("-g", args.resource_group as string);
      return formatResult(await executeIBMCloud(cmdArgs));
    }
    case "ibmcloud_cr_images": {
      const cmdArgs = ["cr", "images"];
      if (args.repository) cmdArgs.push("--repository", args.repository as string);
      if (args.include_ibm) cmdArgs.push("--include-ibm");
      return formatResult(await executeIBMCloud(cmdArgs));
    }
    case "ibmcloud_cr_image_rm": {
      const cmdArgs = ["cr", "image-rm", args.image as string];
      if (args.force) cmdArgs.push("-f");
      return formatResult(await executeIBMCloud(cmdArgs));
    }
    case "ibmcloud_cr_quota":
      return formatResult(await executeIBMCloud(["cr", "quota"]));
    default:
      throw new Error(`Unknown KS tool: ${name}`);
  }
}

async function handleIAMTool(name: string, args: Record<string, unknown>): Promise<string> {
  switch (name) {
    case "ibmcloud_iam_users":
      return formatResult(await executeIBMCloud(["iam", "users"]));
    case "ibmcloud_iam_user_invite": {
      const cmdArgs = ["iam", "user-invite", args.email as string];
      if (args.access_groups) cmdArgs.push("--access-groups", args.access_groups as string);
      return formatResult(await executeIBMCloud(cmdArgs));
    }
    case "ibmcloud_iam_api_keys":
      return formatResult(await executeIBMCloud(["iam", "api-keys"]));
    case "ibmcloud_iam_api_key_create": {
      const cmdArgs = ["iam", "api-key-create", args.name as string];
      if (args.description) cmdArgs.push("-d", args.description as string);
      if (args.file) cmdArgs.push("--file", args.file as string);
      return formatResult(await executeIBMCloud(cmdArgs));
    }
    case "ibmcloud_iam_api_key_delete": {
      const cmdArgs = ["iam", "api-key-delete", args.name as string];
      if (args.force) cmdArgs.push("-f");
      return formatResult(await executeIBMCloud(cmdArgs));
    }
    case "ibmcloud_iam_service_ids":
      return formatResult(await executeIBMCloud(["iam", "service-ids"]));
    case "ibmcloud_iam_service_id":
      return formatResult(await executeIBMCloud(["iam", "service-id", args.name as string]));
    case "ibmcloud_iam_service_id_create": {
      const cmdArgs = ["iam", "service-id-create", args.name as string];
      if (args.description) cmdArgs.push("-d", args.description as string);
      return formatResult(await executeIBMCloud(cmdArgs));
    }
    case "ibmcloud_iam_service_id_delete": {
      const cmdArgs = ["iam", "service-id-delete", args.name as string];
      if (args.force) cmdArgs.push("-f");
      return formatResult(await executeIBMCloud(cmdArgs));
    }
    case "ibmcloud_iam_service_api_keys":
      return formatResult(await executeIBMCloud(["iam", "service-api-keys", args.service_id as string]));
    case "ibmcloud_iam_service_api_key_create": {
      const cmdArgs = ["iam", "service-api-key-create", args.name as string, args.service_id as string];
      if (args.description) cmdArgs.push("-d", args.description as string);
      if (args.file) cmdArgs.push("--file", args.file as string);
      return formatResult(await executeIBMCloud(cmdArgs));
    }
    case "ibmcloud_iam_access_groups":
      return formatResult(await executeIBMCloud(["iam", "access-groups"]));
    case "ibmcloud_iam_access_group":
      return formatResult(await executeIBMCloud(["iam", "access-group", args.name as string]));
    case "ibmcloud_iam_access_group_create": {
      const cmdArgs = ["iam", "access-group-create", args.name as string];
      if (args.description) cmdArgs.push("-d", args.description as string);
      return formatResult(await executeIBMCloud(cmdArgs));
    }
    case "ibmcloud_iam_access_group_users":
      return formatResult(await executeIBMCloud(["iam", "access-group-users", args.group as string]));
    case "ibmcloud_iam_access_group_user_add":
      return formatResult(await executeIBMCloud(["iam", "access-group-user-add", args.group as string, "--users", args.users as string]));
    case "ibmcloud_iam_access_group_policies":
      return formatResult(await executeIBMCloud(["iam", "access-group-policies", args.group as string]));
    case "ibmcloud_iam_access_group_policy_create": {
      const cmdArgs = ["iam", "access-group-policy-create", args.group as string, "--roles", args.roles as string];
      if (args.service_name) cmdArgs.push("--service-name", args.service_name as string);
      if (args.resource_group) cmdArgs.push("--resource-group-name", args.resource_group as string);
      if (args.resource_type) cmdArgs.push("--resource-type", args.resource_type as string);
      if (args.resource) cmdArgs.push("--resource", args.resource as string);
      return formatResult(await executeIBMCloud(cmdArgs));
    }
    case "ibmcloud_iam_roles": {
      const cmdArgs = ["iam", "roles"];
      if (args.service) cmdArgs.push("--service", args.service as string);
      return formatResult(await executeIBMCloud(cmdArgs));
    }
    default:
      throw new Error(`Unknown IAM tool: ${name}`);
  }
}

async function handleCatalogBillingTool(name: string, args: Record<string, unknown>): Promise<string> {
  switch (name) {
    case "ibmcloud_catalog_search":
      return formatResult(await executeIBMCloud(["catalog", "search", args.query as string]));
    case "ibmcloud_catalog_service":
      return formatResult(await executeIBMCloud(["catalog", "service", args.service as string]));
    case "ibmcloud_catalog_service_plans":
      return formatResult(await executeIBMCloud(["catalog", "service", args.service as string]));
    case "ibmcloud_billing_account_usage": {
      const cmdArgs = ["billing", "account-usage"];
      if (args.month) cmdArgs.push("-d", args.month as string);
      return formatResult(await executeIBMCloud(cmdArgs));
    }
    case "ibmcloud_billing_resource_group_usage": {
      const cmdArgs = ["billing", "resource-group-usage", args.resource_group as string];
      if (args.month) cmdArgs.push("-d", args.month as string);
      return formatResult(await executeIBMCloud(cmdArgs));
    }
    case "ibmcloud_billing_resource_instances_usage": {
      const cmdArgs = ["billing", "resource-instances-usage"];
      if (args.month) cmdArgs.push("-d", args.month as string);
      if (args.resource_group) cmdArgs.push("-g", args.resource_group as string);
      return formatResult(await executeIBMCloud(cmdArgs));
    }
    case "ibmcloud_billing_org_usage": {
      const cmdArgs = ["billing", "org-usage", args.org as string];
      if (args.month) cmdArgs.push("-d", args.month as string);
      return formatResult(await executeIBMCloud(cmdArgs));
    }
    case "ibmcloud_plugin_list":
      return formatResult(await executeIBMCloud(["plugin", "list"]));
    case "ibmcloud_plugin_repo_plugins":
      return formatResult(await executeIBMCloud(["plugin", "repo-plugins"]));
    case "ibmcloud_plugin_install": {
      const cmdArgs = ["plugin", "install", args.plugin as string];
      if (args.force) cmdArgs.push("-f");
      return formatResult(await executeIBMCloud(cmdArgs));
    }
    case "ibmcloud_version":
      return formatResult(await executeIBMCloud(["version"]));
    default:
      throw new Error(`Unknown catalog/billing tool: ${name}`);
  }
}

// Main server setup
const server = new Server(
  {
    name: "ibmcloud-mcp",
    version: "1.0.0",
  },
  {
    capabilities: {
      tools: {},
    },
  }
);

// Register tool list handler
server.setRequestHandler(ListToolsRequestSchema, async () => ({
  tools: allTools,
}));

// Register tool call handler
server.setRequestHandler(CallToolRequestSchema, async (request) => {
  const { name, arguments: args } = request.params;
  const typedArgs = (args ?? {}) as Record<string, unknown>;

  try {
    let result: string;

    if (authTools.find((t) => t.name === name)) {
      result = await handleAuthTool(name, typedArgs);
    } else if (resourceTools.find((t) => t.name === name)) {
      result = await handleResourceTool(name, typedArgs);
    } else if (cfTools.find((t) => t.name === name)) {
      result = await handleCFTool(name, typedArgs);
    } else if (ksTools.find((t) => t.name === name)) {
      result = await handleKSTool(name, typedArgs);
    } else if (iamTools.find((t) => t.name === name)) {
      result = await handleIAMTool(name, typedArgs);
    } else if (catalogBillingTools.find((t) => t.name === name)) {
      result = await handleCatalogBillingTool(name, typedArgs);
    } else {
      throw new Error(`Unknown tool: ${name}`);
    }

    return {
      content: [{ type: "text", text: result }],
    };
  } catch (error) {
    const errorMessage = error instanceof Error ? error.message : String(error);
    return {
      content: [{ type: "text", text: `Error: ${errorMessage}` }],
      isError: true,
    };
  }
});

// Start server
async function main() {
  const transport = new StdioServerTransport();
  await server.connect(transport);
  console.error("IBM Cloud MCP Server running on stdio");
}

main().catch(console.error);
