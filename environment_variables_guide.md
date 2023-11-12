# Environment Variables and rcfiles Guide

## Environment Variables Overview

Environment variables are dynamic-named values that affect the processes running on an operating system. They are used to store configuration settings, paths, and other important information. Here's a step-by-step guide on how to work with environment variables.

## 1. Setting Environment Variables:

### On Linux/Mac

**Temporary (for the current session):**

```bash
export VARIABLE_NAME=value
```

**Permanent (across sessions):**
Add the export command to your shell profile file (e.g., ~/.bashrc, ~/.zshrc).

```bash
echo 'export VARIABLE_NAME=value' >> ~/.bashrc
source ~/.bashrc
```

### On Windows

**Temporary (for the current session):**

```powershell
$env:VARIABLE_NAME = "value"
```

**Permanent (across sessions):**
Use the System Properties window to set user or system environment variables.

## 2. Using Environment Variables in rcfiles:

### Example: Bash Shell (~/.bashrc or ~/.bash_profile)

```bash
# Example: Adding a custom directory to the PATH
export PATH=$PATH:/path/to/custom/directory

# Example: Setting a default editor
export EDITOR=vim
```

### Example: PowerShell Profile ($PROFILE)

```powershell
# Example: Adding a custom directory to the PATH
$env:PATH += ";C:\path\to\custom\directory"

# Example: Setting a default editor
$env:EDITOR = "notepad.exe"
```

## 3. Common Configurations:

### Node.js Development

```bash
export NODE_ENV=development
export PORT=3000
```

### Python Development

```bash
export PYTHONPATH=/path/to/project
export FLASK_ENV=development
```

## 4. Windows Considerations:

**On Windows, environment variables can be set through the System Properties or using PowerShell.**
**To persistently modify environment variables on Windows, use the [System Properties](https://www.computerhope.com/issues/ch000549.htm) window. This requires administrative privileges.**

## 5. Security Considerations:

**Avoid storing sensitive information directly in environment variables.**
**On Windows, be cautious about using environment variables in scripts as they can be easily accessed.**

## 6. Best Practices:

**Use uppercase letters and underscores for variable names (e.g., DATABASE_URL).**
**Document your environment variables in a README file for better collaboration.**

By following these steps and examples, you can effectively manage and use environment variables in various scenarios, making your development environment more flexible and scalable.
