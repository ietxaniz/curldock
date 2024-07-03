# CurlDock Technical Specification

## Overview
CurlDock is a lightweight, developer-friendly tool designed to simplify API testing and curl script management. It is built with a focus on simplicity, ease of use, and seamless integration with existing developer workflows.

## Core Principles
1. Simplicity: CurlDock prioritizes ease of use over complex features.
2. Local-First: Primarily designed for use in local development environments.
3. Git-Friendly: Integrates well with version control workflows.
4. No Authentication: Reduces complexity and improves usability.
5. Developer Responsibility: Users are responsible for their own security measures.

## Intended Use
- Local API testing and development
- Management of curl scripts within project repositories
- Collaborative API testing through shared Git repositories

## Security Considerations
- CurlDock does not implement authentication by design.
- It is intended for use in controlled, local environments.
- Users are responsible for ensuring their CurlDock instance is not exposed to untrusted networks.
- Sensitive information should be managed through environment variables or secure vaults, not hardcoded in scripts.

## Git Integration
- CurlDock is designed to work with curl scripts stored in version-controlled directories (e.g., 'rest-examples' folder in a project).
- This approach leverages existing version control systems for script sharing and collaboration.

## Deployment
- Primarily intended for local deployment.
- Can be containerized using Docker for consistent environments.
- Not designed or recommended for public-facing deployments.

## Data Management
- Curl scripts are stored as plain text files.
- File system is used for persistence, aligning with Git-based workflows.
- No built-in database to maintain simplicity.

## Interoperability
- Scripts created in CurlDock can be run directly as shell scripts.
- Export functionality to be considered for compatibility with other API tools.

## Future Considerations
- Maintain focus on simplicity and developer-friendly features.
- Avoid feature creep that could complicate the tool's usage.
- Consider Docker support for easier deployment and isolation.
- Explore features that enhance Git integration and collaboration through version control.

## Non-Goals
- Multi-user authentication system
- Complex access controls
- Built-in security features for public deployments

## User Responsibilities
- Securing their local development environment
- Managing sensitive information (e.g., API keys) securely
- Ensuring CurlDock is not exposed to untrusted networks
- Following best practices for script management and version control

