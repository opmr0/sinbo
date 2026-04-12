# Security Policy

## Reporting a Vulnerability

If you discover a security vulnerability, **do not open a public issue**.

Email: your@email.com

Include a description, steps to reproduce, and potential impact. I'll respond as soon as possible.

## Supported Versions

Only the latest version of sinbo receives security fixes.

## Install Script

The install script at `install.sh` and `install.ps1` should always be reviewed before piping to a shell. You can inspect them directly:

```bash
curl -sSf https://raw.githubusercontent.com/opmr0/sinbo/main/install.sh | less
```

## Scope

- Vulnerabilities in sinbo's encryption implementation
- Supply chain or install script tampering
- Sensitive data exposure via metadata or plaintext leaks