# GitHub Secrets for HyprQur'an

## Introduction

This document outlines the necessary GitHub Secrets for the HyprQur'an project's CI/CD pipeline. GitHub Secrets are encrypted environment variables used in GitHub Actions workflows, typically for storing sensitive data like API keys, deployment credentials, or other tokens.

## Required Secrets

As of the current version, the HyprQur'an application **does not require any GitHub Secrets**.

## Rationale

The project is a self-contained desktop application with the following characteristics:

*   **No External API Integrations:** The application does not communicate with any third-party services that would require API keys or authentication tokens.
*   **No Automated Deployment:** There is currently no automated deployment pipeline configured that would require credentials for package registries, app stores, or servers.
*   **Local Build and Test:** The build and test processes are entirely local and do not involve any secret information.

## Future Considerations

If a CI/CD pipeline is implemented in the future for automated builds, releases, or deployments, this document should be updated. Potential future scenarios requiring secrets include:

*   **Automated GitHub Releases:** A `GITHUB_TOKEN` (which is automatically provided by GitHub Actions) would be needed to create releases and upload artifacts.
*   **Deployment to Package Managers:** If the application is packaged for formats like Flatpak, Snap, or distribution-specific repositories (e.g., AUR, PPA), credentials for these services might be stored as secrets.
*   **Code Signing:** If the application binaries are to be signed, the signing keys or certificates might be stored securely as secrets.

For now, no action is needed regarding GitHub Secrets.
