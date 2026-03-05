#![recursion_limit = "512"]

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::io::{self, BufRead, Write};
use std::process::Command;

#[derive(Deserialize)]
struct JsonRpcRequest {
    #[allow(dead_code)]
    jsonrpc: String,
    id: Option<Value>,
    method: String,
    #[serde(default)]
    params: Value,
}

#[derive(Serialize)]
struct JsonRpcResponse {
    jsonrpc: String,
    id: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<Value>,
}

fn run_ibmcloud(args: &[&str]) -> Result<String, String> {
    // Auto-add --output json for commands that support it
    let json_commands = ["account", "resource", "iam", "catalog", "target", "regions", "api"];
    let should_add_json = !args.is_empty()
        && json_commands.contains(&args[0])
        && !args.iter().any(|a| *a == "--output" || *a == "-o");

    let mut cmd_args: Vec<&str> = args.to_vec();
    if should_add_json {
        cmd_args.push("--output");
        cmd_args.push("json");
    }

    let output = Command::new("ibmcloud")
        .args(&cmd_args)
        .output()
        .map_err(|e| format!("Failed to run ibmcloud: {e}"))?;

    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();

    if output.status.success() {
        Ok(if stdout.is_empty() { "Command completed successfully".into() } else { stdout })
    } else {
        let mut msg = String::new();
        if !stdout.is_empty() { msg.push_str(&stdout); msg.push('\n'); }
        if !stderr.is_empty() { msg.push_str("Error: "); msg.push_str(&stderr); }
        if msg.is_empty() { msg = format!("Command failed with exit code {}", output.status.code().unwrap_or(-1)); }
        Err(msg)
    }
}

fn str_arg<'a>(args: &'a Value, key: &str) -> Option<&'a str> {
    args.get(key).and_then(|v| v.as_str())
}

fn bool_arg(args: &Value, key: &str) -> bool {
    args.get(key).and_then(|v| v.as_bool()).unwrap_or(false)
}

fn call_tool(name: &str, args: &Value) -> Result<String, String> {
    match name {
        // === Auth Tools ===
        "ibmcloud_login" => {
            let mut a: Vec<&str> = vec!["login"];
            let apikey = str_arg(args, "apikey");
            let region = str_arg(args, "region");
            if let Some(k) = apikey { a.push("--apikey"); a.push(k); }
            if bool_arg(args, "sso") { a.push("--sso"); }
            if let Some(r) = region { a.push("-r"); a.push(r); }
            run_ibmcloud(&a)
        }
        "ibmcloud_logout" => run_ibmcloud(&["logout"]),
        "ibmcloud_target" => {
            let mut a: Vec<&str> = vec!["target"];
            let region = str_arg(args, "region");
            let rg = str_arg(args, "resource_group");
            let org = str_arg(args, "org");
            let space = str_arg(args, "space");
            if let Some(v) = region { a.push("-r"); a.push(v); }
            if let Some(v) = rg { a.push("-g"); a.push(v); }
            if let Some(v) = org { a.push("-o"); a.push(v); }
            if let Some(v) = space { a.push("-s"); a.push(v); }
            run_ibmcloud(&a)
        }
        "ibmcloud_api" => {
            let mut a: Vec<&str> = vec!["api"];
            if let Some(ep) = str_arg(args, "endpoint") { a.push(ep); }
            run_ibmcloud(&a)
        }
        "ibmcloud_regions" => run_ibmcloud(&["regions"]),
        "ibmcloud_account_show" => run_ibmcloud(&["account", "show"]),
        "ibmcloud_account_list" => run_ibmcloud(&["account", "list"]),
        "ibmcloud_config_list" => run_ibmcloud(&["config", "list"]),

        // === Resource Tools ===
        "ibmcloud_resource_groups" => run_ibmcloud(&["resource", "groups"]),
        "ibmcloud_resource_group_create" => {
            let n = str_arg(args, "name").unwrap_or("");
            run_ibmcloud(&["resource", "group-create", n])
        }
        "ibmcloud_resource_service_instances" => {
            let mut a: Vec<&str> = vec!["resource", "service-instances"];
            let sn = str_arg(args, "service_name");
            let rg = str_arg(args, "resource_group");
            if let Some(v) = sn { a.push("--service-name"); a.push(v); }
            if let Some(v) = rg { a.push("-g"); a.push(v); }
            run_ibmcloud(&a)
        }
        "ibmcloud_resource_service_instance" => {
            let n = str_arg(args, "name").unwrap_or("");
            run_ibmcloud(&["resource", "service-instance", n])
        }
        "ibmcloud_resource_service_instance_create" => {
            let n = str_arg(args, "name").unwrap_or("");
            let svc = str_arg(args, "service").unwrap_or("");
            let plan = str_arg(args, "plan").unwrap_or("");
            let mut a: Vec<&str> = vec!["resource", "service-instance-create", n, svc, plan];
            let loc = str_arg(args, "location");
            let rg = str_arg(args, "resource_group");
            let params = str_arg(args, "parameters");
            if let Some(v) = loc { a.push("-l"); a.push(v); }
            if let Some(v) = rg { a.push("-g"); a.push(v); }
            if let Some(v) = params { a.push("-p"); a.push(v); }
            run_ibmcloud(&a)
        }
        "ibmcloud_resource_service_instance_delete" => {
            let n = str_arg(args, "name").unwrap_or("");
            let mut a: Vec<&str> = vec!["resource", "service-instance-delete", n];
            if bool_arg(args, "force") { a.push("-f"); }
            run_ibmcloud(&a)
        }
        "ibmcloud_resource_service_instance_update" => {
            let n = str_arg(args, "name").unwrap_or("");
            let mut a: Vec<&str> = vec!["resource", "service-instance-update", n];
            let nn = str_arg(args, "new_name");
            let plan = str_arg(args, "plan");
            let params = str_arg(args, "parameters");
            if let Some(v) = nn { a.push("-n"); a.push(v); }
            if let Some(v) = plan { a.push("--service-plan-id"); a.push(v); }
            if let Some(v) = params { a.push("-p"); a.push(v); }
            run_ibmcloud(&a)
        }
        "ibmcloud_resource_service_keys" => {
            let mut a: Vec<&str> = vec!["resource", "service-keys"];
            let inst = str_arg(args, "instance");
            if let Some(v) = inst { a.push("--instance-name"); a.push(v); }
            run_ibmcloud(&a)
        }
        "ibmcloud_resource_service_key" => {
            let n = str_arg(args, "name").unwrap_or("");
            run_ibmcloud(&["resource", "service-key", n])
        }
        "ibmcloud_resource_service_key_create" => {
            let n = str_arg(args, "name").unwrap_or("");
            let inst = str_arg(args, "instance").unwrap_or("");
            let mut a: Vec<&str> = vec!["resource", "service-key-create", n, "--instance-name", inst];
            let role = str_arg(args, "role");
            let params = str_arg(args, "parameters");
            if let Some(v) = role { a.push("--service-endpoint"); a.push(v); }
            if let Some(v) = params { a.push("-p"); a.push(v); }
            run_ibmcloud(&a)
        }
        "ibmcloud_resource_service_key_delete" => {
            let n = str_arg(args, "name").unwrap_or("");
            let mut a: Vec<&str> = vec!["resource", "service-key-delete", n];
            if bool_arg(args, "force") { a.push("-f"); }
            run_ibmcloud(&a)
        }
        "ibmcloud_resource_search" => {
            let q = str_arg(args, "query").unwrap_or("");
            run_ibmcloud(&["resource", "search", q])
        }
        "ibmcloud_resource_tags" => {
            let mut a: Vec<&str> = vec!["resource", "tags"];
            if str_arg(args, "resource_id").is_some() { a.push("--tag-type"); a.push("user"); }
            run_ibmcloud(&a)
        }
        "ibmcloud_resource_tag_attach" => {
            let tags = str_arg(args, "tags").unwrap_or("");
            let rid = str_arg(args, "resource_id").unwrap_or("");
            run_ibmcloud(&["resource", "tag-attach", "--tag-names", tags, "--resource-id", rid])
        }

        // === Cloud Foundry Tools ===
        "ibmcloud_cf_orgs" => run_ibmcloud(&["cf", "orgs"]),
        "ibmcloud_cf_spaces" => {
            let mut a: Vec<&str> = vec!["cf", "spaces"];
            let org = str_arg(args, "org");
            if let Some(v) = org { a.push("-o"); a.push(v); }
            run_ibmcloud(&a)
        }
        "ibmcloud_cf_apps" => run_ibmcloud(&["cf", "apps"]),
        "ibmcloud_cf_app" => {
            let n = str_arg(args, "name").unwrap_or("");
            run_ibmcloud(&["cf", "app", n])
        }
        "ibmcloud_cf_push" => {
            let mut a: Vec<&str> = vec!["cf", "push"];
            let name = str_arg(args, "name");
            let path = str_arg(args, "path");
            let manifest = str_arg(args, "manifest");
            let memory = str_arg(args, "memory");
            let buildpack = str_arg(args, "buildpack");
            let docker = str_arg(args, "docker_image");
            if let Some(v) = name { a.push(v); }
            if let Some(v) = path { a.push("-p"); a.push(v); }
            if let Some(v) = manifest { a.push("-f"); a.push(v); }
            if let Some(v) = memory { a.push("-m"); a.push(v); }
            if let Some(i) = args.get("instances").and_then(|v| v.as_u64()) {
                let s = i.to_string();
                let leaked: &str = Box::leak(s.into_boxed_str());
                a.push("-i"); a.push(leaked);
            }
            if let Some(v) = buildpack { a.push("-b"); a.push(v); }
            if let Some(v) = docker { a.push("--docker-image"); a.push(v); }
            run_ibmcloud(&a)
        }
        "ibmcloud_cf_start" => {
            let n = str_arg(args, "name").unwrap_or("");
            run_ibmcloud(&["cf", "start", n])
        }
        "ibmcloud_cf_stop" => {
            let n = str_arg(args, "name").unwrap_or("");
            run_ibmcloud(&["cf", "stop", n])
        }
        "ibmcloud_cf_restart" => {
            let n = str_arg(args, "name").unwrap_or("");
            run_ibmcloud(&["cf", "restart", n])
        }
        "ibmcloud_cf_delete" => {
            let n = str_arg(args, "name").unwrap_or("");
            let mut a: Vec<&str> = vec!["cf", "delete", n];
            if bool_arg(args, "force") { a.push("-f"); }
            if bool_arg(args, "delete_routes") { a.push("-r"); }
            run_ibmcloud(&a)
        }
        "ibmcloud_cf_logs" => {
            let n = str_arg(args, "name").unwrap_or("");
            let mut a: Vec<&str> = vec!["cf", "logs", n];
            if !bool_arg(args, "stream") { a.push("--recent"); }
            run_ibmcloud(&a)
        }
        "ibmcloud_cf_env" => {
            let n = str_arg(args, "name").unwrap_or("");
            run_ibmcloud(&["cf", "env", n])
        }
        "ibmcloud_cf_set_env" => {
            let n = str_arg(args, "name").unwrap_or("");
            let vn = str_arg(args, "var_name").unwrap_or("");
            let vv = str_arg(args, "var_value").unwrap_or("");
            run_ibmcloud(&["cf", "set-env", n, vn, vv])
        }
        "ibmcloud_cf_scale" => {
            let n = str_arg(args, "name").unwrap_or("");
            let mut a: Vec<&str> = vec!["cf", "scale", n];
            if let Some(i) = args.get("instances").and_then(|v| v.as_u64()) {
                let s = i.to_string();
                let leaked: &str = Box::leak(s.into_boxed_str());
                a.push("-i"); a.push(leaked);
            }
            let mem = str_arg(args, "memory");
            let disk = str_arg(args, "disk");
            if let Some(v) = mem { a.push("-m"); a.push(v); }
            if let Some(v) = disk { a.push("-k"); a.push(v); }
            a.push("-f");
            run_ibmcloud(&a)
        }
        "ibmcloud_cf_routes" => run_ibmcloud(&["cf", "routes"]),
        "ibmcloud_cf_services" => run_ibmcloud(&["cf", "services"]),
        "ibmcloud_cf_marketplace" => {
            let mut a: Vec<&str> = vec!["cf", "marketplace"];
            let svc = str_arg(args, "service");
            if let Some(v) = svc { a.push("-e"); a.push(v); }
            run_ibmcloud(&a)
        }
        "ibmcloud_cf_create_service" => {
            let svc = str_arg(args, "service").unwrap_or("");
            let plan = str_arg(args, "plan").unwrap_or("");
            let inst = str_arg(args, "instance_name").unwrap_or("");
            let mut a: Vec<&str> = vec!["cf", "create-service", svc, plan, inst];
            let params = str_arg(args, "parameters");
            if let Some(v) = params { a.push("-c"); a.push(v); }
            run_ibmcloud(&a)
        }
        "ibmcloud_cf_bind_service" => {
            let app = str_arg(args, "app_name").unwrap_or("");
            let si = str_arg(args, "service_instance").unwrap_or("");
            let mut a: Vec<&str> = vec!["cf", "bind-service", app, si];
            let params = str_arg(args, "parameters");
            if let Some(v) = params { a.push("-c"); a.push(v); }
            run_ibmcloud(&a)
        }
        "ibmcloud_cf_unbind_service" => {
            let app = str_arg(args, "app_name").unwrap_or("");
            let si = str_arg(args, "service_instance").unwrap_or("");
            run_ibmcloud(&["cf", "unbind-service", app, si])
        }

        // === Kubernetes Service Tools ===
        "ibmcloud_ks_clusters" => {
            let mut a: Vec<&str> = vec!["ks", "clusters"];
            let prov = str_arg(args, "provider");
            if let Some(v) = prov { a.push("--provider"); a.push(v); }
            run_ibmcloud(&a)
        }
        "ibmcloud_ks_cluster" => {
            let c = str_arg(args, "cluster").unwrap_or("");
            run_ibmcloud(&["ks", "cluster", "get", "--cluster", c])
        }
        "ibmcloud_ks_cluster_config" => {
            let c = str_arg(args, "cluster").unwrap_or("");
            let mut a: Vec<&str> = vec!["ks", "cluster", "config", "--cluster", c];
            if bool_arg(args, "admin") { a.push("--admin"); }
            run_ibmcloud(&a)
        }
        "ibmcloud_ks_cluster_create" => {
            let n = str_arg(args, "name").unwrap_or("");
            let z = str_arg(args, "zone").unwrap_or("");
            let vpc = str_arg(args, "vpc_id");
            let infra = if vpc.is_some() { "vpc-gen2" } else { "classic" };
            let mut a: Vec<&str> = vec!["ks", "cluster", "create", infra, "--name", n, "--zone", z];
            let flavor = str_arg(args, "flavor");
            let version = str_arg(args, "version");
            let subnet = str_arg(args, "subnet_id");
            if let Some(v) = flavor { a.push("--flavor"); a.push(v); }
            if let Some(w) = args.get("workers").and_then(|v| v.as_u64()) {
                let s = w.to_string();
                let leaked: &str = Box::leak(s.into_boxed_str());
                a.push("--workers"); a.push(leaked);
            }
            if let Some(v) = version { a.push("--version"); a.push(v); }
            if let Some(v) = vpc { a.push("--vpc-id"); a.push(v); }
            if let Some(v) = subnet { a.push("--subnet-id"); a.push(v); }
            run_ibmcloud(&a)
        }
        "ibmcloud_ks_cluster_delete" => {
            let c = str_arg(args, "cluster").unwrap_or("");
            let mut a: Vec<&str> = vec!["ks", "cluster", "rm", "--cluster", c];
            if bool_arg(args, "force") { a.push("-f"); }
            run_ibmcloud(&a)
        }
        "ibmcloud_ks_workers" => {
            let c = str_arg(args, "cluster").unwrap_or("");
            run_ibmcloud(&["ks", "workers", "--cluster", c])
        }
        "ibmcloud_ks_worker_pools" => {
            let c = str_arg(args, "cluster").unwrap_or("");
            run_ibmcloud(&["ks", "worker-pools", "--cluster", c])
        }
        "ibmcloud_ks_worker_pool_create" => {
            let c = str_arg(args, "cluster").unwrap_or("");
            let n = str_arg(args, "name").unwrap_or("");
            let f = str_arg(args, "flavor").unwrap_or("");
            let mut a: Vec<&str> = vec!["ks", "worker-pool", "create", "classic", "--cluster", c, "--name", n, "--flavor", f];
            if let Some(sz) = args.get("size_per_zone").and_then(|v| v.as_u64()) {
                let s = sz.to_string();
                let leaked: &str = Box::leak(s.into_boxed_str());
                a.push("--size-per-zone"); a.push(leaked);
            }
            run_ibmcloud(&a)
        }
        "ibmcloud_ks_worker_pool_resize" => {
            let c = str_arg(args, "cluster").unwrap_or("");
            let p = str_arg(args, "pool").unwrap_or("");
            let sz = args.get("size_per_zone").and_then(|v| v.as_u64()).unwrap_or(1).to_string();
            let leaked: &str = Box::leak(sz.into_boxed_str());
            run_ibmcloud(&["ks", "worker-pool", "resize", "--cluster", c, "--worker-pool", p, "--size-per-zone", leaked])
        }
        "ibmcloud_ks_zones" => {
            let mut a: Vec<&str> = vec!["ks", "zones"];
            let prov = str_arg(args, "provider");
            let loc = str_arg(args, "location");
            if let Some(v) = prov { a.push("--provider"); a.push(v); }
            if let Some(v) = loc { a.push("--location"); a.push(v); }
            run_ibmcloud(&a)
        }
        "ibmcloud_ks_versions" => run_ibmcloud(&["ks", "versions"]),
        "ibmcloud_ks_flavors" => {
            let z = str_arg(args, "zone").unwrap_or("");
            let mut a: Vec<&str> = vec!["ks", "flavors", "--zone", z];
            let prov = str_arg(args, "provider");
            if let Some(v) = prov { a.push("--provider"); a.push(v); }
            run_ibmcloud(&a)
        }

        // === Container Registry Tools ===
        "ibmcloud_cr_namespaces" => run_ibmcloud(&["cr", "namespaces"]),
        "ibmcloud_cr_namespace_add" => {
            let n = str_arg(args, "name").unwrap_or("");
            let mut a: Vec<&str> = vec!["cr", "namespace-add", n];
            let rg = str_arg(args, "resource_group");
            if let Some(v) = rg { a.push("-g"); a.push(v); }
            run_ibmcloud(&a)
        }
        "ibmcloud_cr_images" => {
            let mut a: Vec<&str> = vec!["cr", "images"];
            let repo = str_arg(args, "repository");
            if let Some(v) = repo { a.push("--repository"); a.push(v); }
            if bool_arg(args, "include_ibm") { a.push("--include-ibm"); }
            run_ibmcloud(&a)
        }
        "ibmcloud_cr_image_rm" => {
            let img = str_arg(args, "image").unwrap_or("");
            let mut a: Vec<&str> = vec!["cr", "image-rm", img];
            if bool_arg(args, "force") { a.push("-f"); }
            run_ibmcloud(&a)
        }
        "ibmcloud_cr_quota" => run_ibmcloud(&["cr", "quota"]),

        // === IAM Tools ===
        "ibmcloud_iam_users" => run_ibmcloud(&["iam", "users"]),
        "ibmcloud_iam_user_invite" => {
            let email = str_arg(args, "email").unwrap_or("");
            let mut a: Vec<&str> = vec!["iam", "user-invite", email];
            let ag = str_arg(args, "access_groups");
            if let Some(v) = ag { a.push("--access-groups"); a.push(v); }
            run_ibmcloud(&a)
        }
        "ibmcloud_iam_api_keys" => run_ibmcloud(&["iam", "api-keys"]),
        "ibmcloud_iam_api_key_create" => {
            let n = str_arg(args, "name").unwrap_or("");
            let mut a: Vec<&str> = vec!["iam", "api-key-create", n];
            let desc = str_arg(args, "description");
            let file = str_arg(args, "file");
            if let Some(v) = desc { a.push("-d"); a.push(v); }
            if let Some(v) = file { a.push("--file"); a.push(v); }
            run_ibmcloud(&a)
        }
        "ibmcloud_iam_api_key_delete" => {
            let n = str_arg(args, "name").unwrap_or("");
            let mut a: Vec<&str> = vec!["iam", "api-key-delete", n];
            if bool_arg(args, "force") { a.push("-f"); }
            run_ibmcloud(&a)
        }
        "ibmcloud_iam_service_ids" => run_ibmcloud(&["iam", "service-ids"]),
        "ibmcloud_iam_service_id" => {
            let n = str_arg(args, "name").unwrap_or("");
            run_ibmcloud(&["iam", "service-id", n])
        }
        "ibmcloud_iam_service_id_create" => {
            let n = str_arg(args, "name").unwrap_or("");
            let mut a: Vec<&str> = vec!["iam", "service-id-create", n];
            let desc = str_arg(args, "description");
            if let Some(v) = desc { a.push("-d"); a.push(v); }
            run_ibmcloud(&a)
        }
        "ibmcloud_iam_service_id_delete" => {
            let n = str_arg(args, "name").unwrap_or("");
            let mut a: Vec<&str> = vec!["iam", "service-id-delete", n];
            if bool_arg(args, "force") { a.push("-f"); }
            run_ibmcloud(&a)
        }
        "ibmcloud_iam_service_api_keys" => {
            let sid = str_arg(args, "service_id").unwrap_or("");
            run_ibmcloud(&["iam", "service-api-keys", sid])
        }
        "ibmcloud_iam_service_api_key_create" => {
            let n = str_arg(args, "name").unwrap_or("");
            let sid = str_arg(args, "service_id").unwrap_or("");
            let mut a: Vec<&str> = vec!["iam", "service-api-key-create", n, sid];
            let desc = str_arg(args, "description");
            let file = str_arg(args, "file");
            if let Some(v) = desc { a.push("-d"); a.push(v); }
            if let Some(v) = file { a.push("--file"); a.push(v); }
            run_ibmcloud(&a)
        }
        "ibmcloud_iam_access_groups" => run_ibmcloud(&["iam", "access-groups"]),
        "ibmcloud_iam_access_group" => {
            let n = str_arg(args, "name").unwrap_or("");
            run_ibmcloud(&["iam", "access-group", n])
        }
        "ibmcloud_iam_access_group_create" => {
            let n = str_arg(args, "name").unwrap_or("");
            let mut a: Vec<&str> = vec!["iam", "access-group-create", n];
            let desc = str_arg(args, "description");
            if let Some(v) = desc { a.push("-d"); a.push(v); }
            run_ibmcloud(&a)
        }
        "ibmcloud_iam_access_group_users" => {
            let g = str_arg(args, "group").unwrap_or("");
            run_ibmcloud(&["iam", "access-group-users", g])
        }
        "ibmcloud_iam_access_group_user_add" => {
            let g = str_arg(args, "group").unwrap_or("");
            let u = str_arg(args, "users").unwrap_or("");
            run_ibmcloud(&["iam", "access-group-user-add", g, "--users", u])
        }
        "ibmcloud_iam_access_group_policies" => {
            let g = str_arg(args, "group").unwrap_or("");
            run_ibmcloud(&["iam", "access-group-policies", g])
        }
        "ibmcloud_iam_access_group_policy_create" => {
            let g = str_arg(args, "group").unwrap_or("");
            let roles = str_arg(args, "roles").unwrap_or("");
            let mut a: Vec<&str> = vec!["iam", "access-group-policy-create", g, "--roles", roles];
            let sn = str_arg(args, "service_name");
            let rg = str_arg(args, "resource_group");
            let rt = str_arg(args, "resource_type");
            let res = str_arg(args, "resource");
            if let Some(v) = sn { a.push("--service-name"); a.push(v); }
            if let Some(v) = rg { a.push("--resource-group-name"); a.push(v); }
            if let Some(v) = rt { a.push("--resource-type"); a.push(v); }
            if let Some(v) = res { a.push("--resource"); a.push(v); }
            run_ibmcloud(&a)
        }
        "ibmcloud_iam_roles" => {
            let mut a: Vec<&str> = vec!["iam", "roles"];
            let svc = str_arg(args, "service");
            if let Some(v) = svc { a.push("--service"); a.push(v); }
            run_ibmcloud(&a)
        }

        // === Catalog & Billing Tools ===
        "ibmcloud_catalog_search" => {
            let q = str_arg(args, "query").unwrap_or("");
            run_ibmcloud(&["catalog", "search", q])
        }
        "ibmcloud_catalog_service" => {
            let s = str_arg(args, "service").unwrap_or("");
            run_ibmcloud(&["catalog", "service", s])
        }
        "ibmcloud_catalog_service_plans" => {
            let s = str_arg(args, "service").unwrap_or("");
            run_ibmcloud(&["catalog", "service", s])
        }
        "ibmcloud_billing_account_usage" => {
            let mut a: Vec<&str> = vec!["billing", "account-usage"];
            let m = str_arg(args, "month");
            if let Some(v) = m { a.push("-d"); a.push(v); }
            run_ibmcloud(&a)
        }
        "ibmcloud_billing_resource_group_usage" => {
            let rg = str_arg(args, "resource_group").unwrap_or("");
            let mut a: Vec<&str> = vec!["billing", "resource-group-usage", rg];
            let m = str_arg(args, "month");
            if let Some(v) = m { a.push("-d"); a.push(v); }
            run_ibmcloud(&a)
        }
        "ibmcloud_billing_resource_instances_usage" => {
            let mut a: Vec<&str> = vec!["billing", "resource-instances-usage"];
            let m = str_arg(args, "month");
            let rg = str_arg(args, "resource_group");
            if let Some(v) = m { a.push("-d"); a.push(v); }
            if let Some(v) = rg { a.push("-g"); a.push(v); }
            run_ibmcloud(&a)
        }
        "ibmcloud_billing_org_usage" => {
            let org = str_arg(args, "org").unwrap_or("");
            let mut a: Vec<&str> = vec!["billing", "org-usage", org];
            let m = str_arg(args, "month");
            if let Some(v) = m { a.push("-d"); a.push(v); }
            run_ibmcloud(&a)
        }

        // === Plugin & Version Tools ===
        "ibmcloud_plugin_list" => run_ibmcloud(&["plugin", "list"]),
        "ibmcloud_plugin_repo_plugins" => run_ibmcloud(&["plugin", "repo-plugins"]),
        "ibmcloud_plugin_install" => {
            let p = str_arg(args, "plugin").unwrap_or("");
            let mut a: Vec<&str> = vec!["plugin", "install", p];
            if bool_arg(args, "force") { a.push("-f"); }
            run_ibmcloud(&a)
        }
        "ibmcloud_version" => run_ibmcloud(&["version"]),

        _ => Err(format!("Unknown tool: {name}")),
    }
}

fn tool_definitions() -> Value {
    json!([
        // Auth Tools
        {"name":"ibmcloud_login","description":"Login to IBM Cloud with API key or SSO","inputSchema":{"type":"object","properties":{"apikey":{"type":"string","description":"IBM Cloud API key"},"sso":{"type":"boolean","description":"Use SSO for login"},"region":{"type":"string","description":"Target region (e.g., us-south)"}}}},
        {"name":"ibmcloud_logout","description":"Logout from IBM Cloud","inputSchema":{"type":"object","properties":{}}},
        {"name":"ibmcloud_target","description":"View or set target region, resource group, org, and space","inputSchema":{"type":"object","properties":{"region":{"type":"string","description":"Target region"},"resource_group":{"type":"string","description":"Target resource group"},"org":{"type":"string","description":"Target Cloud Foundry org"},"space":{"type":"string","description":"Target Cloud Foundry space"}}}},
        {"name":"ibmcloud_api","description":"View or set IBM Cloud API endpoint","inputSchema":{"type":"object","properties":{"endpoint":{"type":"string","description":"API endpoint URL"}}}},
        {"name":"ibmcloud_regions","description":"List all IBM Cloud regions","inputSchema":{"type":"object","properties":{}}},
        {"name":"ibmcloud_account_show","description":"Show current IBM Cloud account details","inputSchema":{"type":"object","properties":{}}},
        {"name":"ibmcloud_account_list","description":"List all available accounts","inputSchema":{"type":"object","properties":{}}},
        {"name":"ibmcloud_config_list","description":"List IBM Cloud CLI configuration","inputSchema":{"type":"object","properties":{}}},
        // Resource Tools
        {"name":"ibmcloud_resource_groups","description":"List all resource groups","inputSchema":{"type":"object","properties":{}}},
        {"name":"ibmcloud_resource_group_create","description":"Create a new resource group","inputSchema":{"type":"object","properties":{"name":{"type":"string","description":"Name of the resource group"}},"required":["name"]}},
        {"name":"ibmcloud_resource_service_instances","description":"List resource service instances","inputSchema":{"type":"object","properties":{"service_name":{"type":"string","description":"Filter by service name"},"resource_group":{"type":"string","description":"Filter by resource group"}}}},
        {"name":"ibmcloud_resource_service_instance","description":"Show details of a service instance","inputSchema":{"type":"object","properties":{"name":{"type":"string","description":"Service instance name"}},"required":["name"]}},
        {"name":"ibmcloud_resource_service_instance_create","description":"Create a service instance","inputSchema":{"type":"object","properties":{"name":{"type":"string","description":"Instance name"},"service":{"type":"string","description":"Service offering"},"plan":{"type":"string","description":"Service plan"},"location":{"type":"string","description":"Target location"},"resource_group":{"type":"string","description":"Resource group"},"parameters":{"type":"string","description":"JSON parameters"}},"required":["name","service","plan"]}},
        {"name":"ibmcloud_resource_service_instance_delete","description":"Delete a service instance","inputSchema":{"type":"object","properties":{"name":{"type":"string","description":"Instance name"},"force":{"type":"boolean","description":"Force deletion"}},"required":["name"]}},
        {"name":"ibmcloud_resource_service_instance_update","description":"Update a service instance","inputSchema":{"type":"object","properties":{"name":{"type":"string","description":"Instance name"},"new_name":{"type":"string","description":"New name"},"plan":{"type":"string","description":"New plan ID"},"parameters":{"type":"string","description":"JSON parameters"}},"required":["name"]}},
        {"name":"ibmcloud_resource_service_keys","description":"List service keys","inputSchema":{"type":"object","properties":{"instance":{"type":"string","description":"Filter by instance name"}}}},
        {"name":"ibmcloud_resource_service_key","description":"Show details of a service key","inputSchema":{"type":"object","properties":{"name":{"type":"string","description":"Service key name"}},"required":["name"]}},
        {"name":"ibmcloud_resource_service_key_create","description":"Create a service key for an instance","inputSchema":{"type":"object","properties":{"name":{"type":"string","description":"Key name"},"instance":{"type":"string","description":"Instance name"},"role":{"type":"string","description":"Service endpoint/role"},"parameters":{"type":"string","description":"JSON parameters"}},"required":["name","instance"]}},
        {"name":"ibmcloud_resource_service_key_delete","description":"Delete a service key","inputSchema":{"type":"object","properties":{"name":{"type":"string","description":"Key name"},"force":{"type":"boolean","description":"Force deletion"}},"required":["name"]}},
        {"name":"ibmcloud_resource_search","description":"Search for resources","inputSchema":{"type":"object","properties":{"query":{"type":"string","description":"Search query"}},"required":["query"]}},
        {"name":"ibmcloud_resource_tags","description":"List resource tags","inputSchema":{"type":"object","properties":{"resource_id":{"type":"string","description":"Resource ID to filter tags"}}}},
        {"name":"ibmcloud_resource_tag_attach","description":"Attach tags to a resource","inputSchema":{"type":"object","properties":{"tags":{"type":"string","description":"Comma-separated tag names"},"resource_id":{"type":"string","description":"Resource CRN"}},"required":["tags","resource_id"]}},
        // Cloud Foundry Tools
        {"name":"ibmcloud_cf_orgs","description":"List Cloud Foundry organizations","inputSchema":{"type":"object","properties":{}}},
        {"name":"ibmcloud_cf_spaces","description":"List Cloud Foundry spaces","inputSchema":{"type":"object","properties":{"org":{"type":"string","description":"Organization name"}}}},
        {"name":"ibmcloud_cf_apps","description":"List all Cloud Foundry applications","inputSchema":{"type":"object","properties":{}}},
        {"name":"ibmcloud_cf_app","description":"Show details of a Cloud Foundry app","inputSchema":{"type":"object","properties":{"name":{"type":"string","description":"App name"}},"required":["name"]}},
        {"name":"ibmcloud_cf_push","description":"Push a Cloud Foundry application","inputSchema":{"type":"object","properties":{"name":{"type":"string","description":"App name"},"path":{"type":"string","description":"App path"},"manifest":{"type":"string","description":"Manifest file path"},"memory":{"type":"string","description":"Memory limit (e.g., 256M)"},"instances":{"type":"number","description":"Number of instances"},"buildpack":{"type":"string","description":"Buildpack name/URL"},"docker_image":{"type":"string","description":"Docker image to deploy"}}}},
        {"name":"ibmcloud_cf_start","description":"Start a Cloud Foundry app","inputSchema":{"type":"object","properties":{"name":{"type":"string","description":"App name"}},"required":["name"]}},
        {"name":"ibmcloud_cf_stop","description":"Stop a Cloud Foundry app","inputSchema":{"type":"object","properties":{"name":{"type":"string","description":"App name"}},"required":["name"]}},
        {"name":"ibmcloud_cf_restart","description":"Restart a Cloud Foundry app","inputSchema":{"type":"object","properties":{"name":{"type":"string","description":"App name"}},"required":["name"]}},
        {"name":"ibmcloud_cf_delete","description":"Delete a Cloud Foundry app","inputSchema":{"type":"object","properties":{"name":{"type":"string","description":"App name"},"force":{"type":"boolean","description":"Force deletion"},"delete_routes":{"type":"boolean","description":"Also delete routes"}},"required":["name"]}},
        {"name":"ibmcloud_cf_logs","description":"Show recent logs for a Cloud Foundry app","inputSchema":{"type":"object","properties":{"name":{"type":"string","description":"App name"},"stream":{"type":"boolean","description":"Stream logs instead of recent"}},"required":["name"]}},
        {"name":"ibmcloud_cf_env","description":"Show environment variables for a CF app","inputSchema":{"type":"object","properties":{"name":{"type":"string","description":"App name"}},"required":["name"]}},
        {"name":"ibmcloud_cf_set_env","description":"Set an environment variable for a CF app","inputSchema":{"type":"object","properties":{"name":{"type":"string","description":"App name"},"var_name":{"type":"string","description":"Variable name"},"var_value":{"type":"string","description":"Variable value"}},"required":["name","var_name","var_value"]}},
        {"name":"ibmcloud_cf_scale","description":"Scale a Cloud Foundry app","inputSchema":{"type":"object","properties":{"name":{"type":"string","description":"App name"},"instances":{"type":"number","description":"Number of instances"},"memory":{"type":"string","description":"Memory limit"},"disk":{"type":"string","description":"Disk limit"}},"required":["name"]}},
        {"name":"ibmcloud_cf_routes","description":"List Cloud Foundry routes","inputSchema":{"type":"object","properties":{}}},
        {"name":"ibmcloud_cf_services","description":"List Cloud Foundry services","inputSchema":{"type":"object","properties":{}}},
        {"name":"ibmcloud_cf_marketplace","description":"List services in the Cloud Foundry marketplace","inputSchema":{"type":"object","properties":{"service":{"type":"string","description":"Show plans for a specific service"}}}},
        {"name":"ibmcloud_cf_create_service","description":"Create a Cloud Foundry service instance","inputSchema":{"type":"object","properties":{"service":{"type":"string","description":"Service offering"},"plan":{"type":"string","description":"Service plan"},"instance_name":{"type":"string","description":"Instance name"},"parameters":{"type":"string","description":"JSON parameters"}},"required":["service","plan","instance_name"]}},
        {"name":"ibmcloud_cf_bind_service","description":"Bind a service to a CF app","inputSchema":{"type":"object","properties":{"app_name":{"type":"string","description":"App name"},"service_instance":{"type":"string","description":"Service instance name"},"parameters":{"type":"string","description":"JSON parameters"}},"required":["app_name","service_instance"]}},
        {"name":"ibmcloud_cf_unbind_service","description":"Unbind a service from a CF app","inputSchema":{"type":"object","properties":{"app_name":{"type":"string","description":"App name"},"service_instance":{"type":"string","description":"Service instance name"}},"required":["app_name","service_instance"]}},
        // Kubernetes Service Tools
        {"name":"ibmcloud_ks_clusters","description":"List Kubernetes clusters","inputSchema":{"type":"object","properties":{"provider":{"type":"string","description":"Infrastructure provider (classic/vpc-gen2)"}}}},
        {"name":"ibmcloud_ks_cluster","description":"Get details of a Kubernetes cluster","inputSchema":{"type":"object","properties":{"cluster":{"type":"string","description":"Cluster name or ID"}},"required":["cluster"]}},
        {"name":"ibmcloud_ks_cluster_config","description":"Download cluster config for kubectl","inputSchema":{"type":"object","properties":{"cluster":{"type":"string","description":"Cluster name or ID"},"admin":{"type":"boolean","description":"Download admin config"}},"required":["cluster"]}},
        {"name":"ibmcloud_ks_cluster_create","description":"Create a Kubernetes cluster","inputSchema":{"type":"object","properties":{"name":{"type":"string","description":"Cluster name"},"zone":{"type":"string","description":"Zone"},"flavor":{"type":"string","description":"Worker node flavor"},"workers":{"type":"number","description":"Number of workers"},"version":{"type":"string","description":"Kubernetes version"},"vpc_id":{"type":"string","description":"VPC ID (uses vpc-gen2 infra)"},"subnet_id":{"type":"string","description":"Subnet ID"}},"required":["name","zone"]}},
        {"name":"ibmcloud_ks_cluster_delete","description":"Delete a Kubernetes cluster","inputSchema":{"type":"object","properties":{"cluster":{"type":"string","description":"Cluster name or ID"},"force":{"type":"boolean","description":"Force deletion"}},"required":["cluster"]}},
        {"name":"ibmcloud_ks_workers","description":"List worker nodes in a cluster","inputSchema":{"type":"object","properties":{"cluster":{"type":"string","description":"Cluster name or ID"}},"required":["cluster"]}},
        {"name":"ibmcloud_ks_worker_pools","description":"List worker pools in a cluster","inputSchema":{"type":"object","properties":{"cluster":{"type":"string","description":"Cluster name or ID"}},"required":["cluster"]}},
        {"name":"ibmcloud_ks_worker_pool_create","description":"Create a worker pool","inputSchema":{"type":"object","properties":{"cluster":{"type":"string","description":"Cluster name or ID"},"name":{"type":"string","description":"Pool name"},"flavor":{"type":"string","description":"Worker flavor"},"size_per_zone":{"type":"number","description":"Workers per zone"}},"required":["cluster","name","flavor"]}},
        {"name":"ibmcloud_ks_worker_pool_resize","description":"Resize a worker pool","inputSchema":{"type":"object","properties":{"cluster":{"type":"string","description":"Cluster name or ID"},"pool":{"type":"string","description":"Pool name"},"size_per_zone":{"type":"number","description":"New size per zone"}},"required":["cluster","pool","size_per_zone"]}},
        {"name":"ibmcloud_ks_zones","description":"List available zones for Kubernetes","inputSchema":{"type":"object","properties":{"provider":{"type":"string","description":"Infrastructure provider"},"location":{"type":"string","description":"Location filter"}}}},
        {"name":"ibmcloud_ks_versions","description":"List supported Kubernetes versions","inputSchema":{"type":"object","properties":{}}},
        {"name":"ibmcloud_ks_flavors","description":"List available worker node flavors","inputSchema":{"type":"object","properties":{"zone":{"type":"string","description":"Zone"},"provider":{"type":"string","description":"Infrastructure provider"}},"required":["zone"]}},
        // Container Registry Tools
        {"name":"ibmcloud_cr_namespaces","description":"List container registry namespaces","inputSchema":{"type":"object","properties":{}}},
        {"name":"ibmcloud_cr_namespace_add","description":"Create a container registry namespace","inputSchema":{"type":"object","properties":{"name":{"type":"string","description":"Namespace name"},"resource_group":{"type":"string","description":"Resource group"}},"required":["name"]}},
        {"name":"ibmcloud_cr_images","description":"List container images","inputSchema":{"type":"object","properties":{"repository":{"type":"string","description":"Repository filter"},"include_ibm":{"type":"boolean","description":"Include IBM images"}}}},
        {"name":"ibmcloud_cr_image_rm","description":"Remove a container image","inputSchema":{"type":"object","properties":{"image":{"type":"string","description":"Image name"},"force":{"type":"boolean","description":"Force removal"}},"required":["image"]}},
        {"name":"ibmcloud_cr_quota","description":"Show container registry quota","inputSchema":{"type":"object","properties":{}}},
        // IAM Tools
        {"name":"ibmcloud_iam_users","description":"List IAM users","inputSchema":{"type":"object","properties":{}}},
        {"name":"ibmcloud_iam_user_invite","description":"Invite a user to the account","inputSchema":{"type":"object","properties":{"email":{"type":"string","description":"User email"},"access_groups":{"type":"string","description":"Comma-separated access group names"}},"required":["email"]}},
        {"name":"ibmcloud_iam_api_keys","description":"List IAM API keys","inputSchema":{"type":"object","properties":{}}},
        {"name":"ibmcloud_iam_api_key_create","description":"Create an IAM API key","inputSchema":{"type":"object","properties":{"name":{"type":"string","description":"Key name"},"description":{"type":"string","description":"Key description"},"file":{"type":"string","description":"File to save key"}},"required":["name"]}},
        {"name":"ibmcloud_iam_api_key_delete","description":"Delete an IAM API key","inputSchema":{"type":"object","properties":{"name":{"type":"string","description":"Key name"},"force":{"type":"boolean","description":"Force deletion"}},"required":["name"]}},
        {"name":"ibmcloud_iam_service_ids","description":"List IAM service IDs","inputSchema":{"type":"object","properties":{}}},
        {"name":"ibmcloud_iam_service_id","description":"Show details of a service ID","inputSchema":{"type":"object","properties":{"name":{"type":"string","description":"Service ID name"}},"required":["name"]}},
        {"name":"ibmcloud_iam_service_id_create","description":"Create a service ID","inputSchema":{"type":"object","properties":{"name":{"type":"string","description":"Service ID name"},"description":{"type":"string","description":"Description"}},"required":["name"]}},
        {"name":"ibmcloud_iam_service_id_delete","description":"Delete a service ID","inputSchema":{"type":"object","properties":{"name":{"type":"string","description":"Service ID name"},"force":{"type":"boolean","description":"Force deletion"}},"required":["name"]}},
        {"name":"ibmcloud_iam_service_api_keys","description":"List API keys for a service ID","inputSchema":{"type":"object","properties":{"service_id":{"type":"string","description":"Service ID"}},"required":["service_id"]}},
        {"name":"ibmcloud_iam_service_api_key_create","description":"Create an API key for a service ID","inputSchema":{"type":"object","properties":{"name":{"type":"string","description":"Key name"},"service_id":{"type":"string","description":"Service ID"},"description":{"type":"string","description":"Description"},"file":{"type":"string","description":"File to save key"}},"required":["name","service_id"]}},
        {"name":"ibmcloud_iam_access_groups","description":"List IAM access groups","inputSchema":{"type":"object","properties":{}}},
        {"name":"ibmcloud_iam_access_group","description":"Show details of an access group","inputSchema":{"type":"object","properties":{"name":{"type":"string","description":"Access group name"}},"required":["name"]}},
        {"name":"ibmcloud_iam_access_group_create","description":"Create an access group","inputSchema":{"type":"object","properties":{"name":{"type":"string","description":"Group name"},"description":{"type":"string","description":"Description"}},"required":["name"]}},
        {"name":"ibmcloud_iam_access_group_users","description":"List users in an access group","inputSchema":{"type":"object","properties":{"group":{"type":"string","description":"Access group name"}},"required":["group"]}},
        {"name":"ibmcloud_iam_access_group_user_add","description":"Add users to an access group","inputSchema":{"type":"object","properties":{"group":{"type":"string","description":"Access group name"},"users":{"type":"string","description":"Comma-separated user emails"}},"required":["group","users"]}},
        {"name":"ibmcloud_iam_access_group_policies","description":"List policies for an access group","inputSchema":{"type":"object","properties":{"group":{"type":"string","description":"Access group name"}},"required":["group"]}},
        {"name":"ibmcloud_iam_access_group_policy_create","description":"Create a policy for an access group","inputSchema":{"type":"object","properties":{"group":{"type":"string","description":"Access group name"},"roles":{"type":"string","description":"Comma-separated roles"},"service_name":{"type":"string","description":"Service name"},"resource_group":{"type":"string","description":"Resource group name"},"resource_type":{"type":"string","description":"Resource type"},"resource":{"type":"string","description":"Resource"}},"required":["group","roles"]}},
        {"name":"ibmcloud_iam_roles","description":"List IAM roles","inputSchema":{"type":"object","properties":{"service":{"type":"string","description":"Filter by service name"}}}},
        // Catalog & Billing Tools
        {"name":"ibmcloud_catalog_search","description":"Search the IBM Cloud catalog","inputSchema":{"type":"object","properties":{"query":{"type":"string","description":"Search query"}},"required":["query"]}},
        {"name":"ibmcloud_catalog_service","description":"Show details of a catalog service","inputSchema":{"type":"object","properties":{"service":{"type":"string","description":"Service name"}},"required":["service"]}},
        {"name":"ibmcloud_catalog_service_plans","description":"List plans for a catalog service","inputSchema":{"type":"object","properties":{"service":{"type":"string","description":"Service name"}},"required":["service"]}},
        {"name":"ibmcloud_billing_account_usage","description":"Show account billing usage","inputSchema":{"type":"object","properties":{"month":{"type":"string","description":"Month (YYYY-MM)"}}}},
        {"name":"ibmcloud_billing_resource_group_usage","description":"Show resource group billing usage","inputSchema":{"type":"object","properties":{"resource_group":{"type":"string","description":"Resource group name"},"month":{"type":"string","description":"Month (YYYY-MM)"}},"required":["resource_group"]}},
        {"name":"ibmcloud_billing_resource_instances_usage","description":"Show billing for resource instances","inputSchema":{"type":"object","properties":{"month":{"type":"string","description":"Month (YYYY-MM)"},"resource_group":{"type":"string","description":"Resource group filter"}}}},
        {"name":"ibmcloud_billing_org_usage","description":"Show Cloud Foundry org billing usage","inputSchema":{"type":"object","properties":{"org":{"type":"string","description":"Organization name"},"month":{"type":"string","description":"Month (YYYY-MM)"}},"required":["org"]}},
        // Plugin & Version Tools
        {"name":"ibmcloud_plugin_list","description":"List installed CLI plugins","inputSchema":{"type":"object","properties":{}}},
        {"name":"ibmcloud_plugin_repo_plugins","description":"List available plugins in the repository","inputSchema":{"type":"object","properties":{}}},
        {"name":"ibmcloud_plugin_install","description":"Install a CLI plugin","inputSchema":{"type":"object","properties":{"plugin":{"type":"string","description":"Plugin name"},"force":{"type":"boolean","description":"Force reinstall"}},"required":["plugin"]}},
        {"name":"ibmcloud_version","description":"Show IBM Cloud CLI version","inputSchema":{"type":"object","properties":{}}}
    ])
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .init();

    tracing::info!("ibmcloud-mcp-server starting");

    let stdin = io::stdin();
    let stdout = io::stdout();

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => break,
        };
        if line.trim().is_empty() { continue; }

        let req: JsonRpcRequest = match serde_json::from_str(&line) {
            Ok(r) => r,
            Err(e) => { tracing::warn!("invalid JSON-RPC: {e}"); continue; }
        };

        let id = req.id.clone().unwrap_or(Value::Null);

        let response = match req.method.as_str() {
            "initialize" => Some(JsonRpcResponse {
                jsonrpc: "2.0".into(), id,
                result: Some(json!({
                    "protocolVersion": "2024-11-05",
                    "capabilities": {"tools": {}},
                    "serverInfo": {"name": "ibmcloud-mcp-server", "version": env!("CARGO_PKG_VERSION")}
                })),
                error: None,
            }),
            "notifications/initialized" => None,
            "tools/list" => Some(JsonRpcResponse {
                jsonrpc: "2.0".into(), id,
                result: Some(json!({"tools": tool_definitions()})),
                error: None,
            }),
            "tools/call" => {
                let name = req.params.get("name").and_then(|v| v.as_str()).unwrap_or("");
                let args = req.params.get("arguments").cloned().unwrap_or(json!({}));
                let result = call_tool(name, &args);
                let content = match result {
                    Ok(text) => json!({"content": [{"type": "text", "text": text}]}),
                    Err(e) => json!({"content": [{"type": "text", "text": format!("Error: {e}")}], "isError": true}),
                };
                Some(JsonRpcResponse {
                    jsonrpc: "2.0".into(), id,
                    result: Some(content),
                    error: None,
                })
            }
            other => Some(JsonRpcResponse {
                jsonrpc: "2.0".into(), id,
                result: None,
                error: Some(json!({"code": -32601, "message": format!("method not found: {other}")})),
            }),
        };

        if let Some(resp) = response {
            let mut out = stdout.lock();
            let _ = serde_json::to_writer(&mut out, &resp);
            let _ = out.write_all(b"\n");
            let _ = out.flush();
        }
    }
}
