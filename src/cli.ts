import { spawn } from "child_process";

export interface CLIResult {
  success: boolean;
  stdout: string;
  stderr: string;
  exitCode: number;
}

export async function executeIBMCloud(
  args: string[],
  options?: {
    timeout?: number;
    env?: Record<string, string>;
  }
): Promise<CLIResult> {
  return new Promise((resolve) => {
    const timeout = options?.timeout ?? 120000; // 2 minute default
    const env = { ...process.env, ...options?.env };

    // Add --output json flag if not already present and command supports it
    const jsonCommands = [
      "account",
      "resource",
      "iam",
      "catalog",
      "target",
      "regions",
      "api",
    ];
    const shouldAddJson =
      args.length > 0 &&
      jsonCommands.some((cmd) => args[0] === cmd) &&
      !args.includes("--output") &&
      !args.includes("-o");

    const finalArgs = shouldAddJson ? [...args, "--output", "json"] : args;

    const child = spawn("ibmcloud", finalArgs, {
      env,
      shell: true,
    });

    let stdout = "";
    let stderr = "";

    child.stdout.on("data", (data) => {
      stdout += data.toString();
    });

    child.stderr.on("data", (data) => {
      stderr += data.toString();
    });

    const timer = setTimeout(() => {
      child.kill("SIGTERM");
      resolve({
        success: false,
        stdout,
        stderr: stderr + "\nCommand timed out",
        exitCode: -1,
      });
    }, timeout);

    child.on("close", (code) => {
      clearTimeout(timer);
      resolve({
        success: code === 0,
        stdout: stdout.trim(),
        stderr: stderr.trim(),
        exitCode: code ?? -1,
      });
    });

    child.on("error", (err) => {
      clearTimeout(timer);
      resolve({
        success: false,
        stdout: "",
        stderr: err.message,
        exitCode: -1,
      });
    });
  });
}

export function formatResult(result: CLIResult): string {
  if (result.success) {
    return result.stdout || "Command completed successfully";
  } else {
    let output = "";
    if (result.stdout) {
      output += result.stdout + "\n";
    }
    if (result.stderr) {
      output += `Error: ${result.stderr}`;
    }
    if (!output) {
      output = `Command failed with exit code ${result.exitCode}`;
    }
    return output;
  }
}
