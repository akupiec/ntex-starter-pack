[![Dependabot][s1]][l1] [![Tests](https://github.com/akupiec/ntex-starter-pack/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/akupiec/ntex-starter-pack/actions/workflows/rust.yml) [![MIT][s2]][l2] [![Version][s3]][l3]

[s1]: https://github.com/akupiec/ntex-rest-starter-pack/actions/workflows/dependabot/dependabot-updates/badge.svg?branch=master
[l1]: https://github.com/akupiec/ntex-starter-pack/actions/workflows/dependabot/dependabot-updates

[s2]: https://img.shields.io/badge/license-MIT-blue.svg
[l2]: LICENSE

[s3]: https://img.shields.io/badge/rustc-1.75+-lightgray.svg
[l3]: https://blog.rust-lang.org/2023/12/28/Rust-1.75.0.html

# Description

Simple monorepo starter pack using fastest & smallest reasonable framework [(check current data here)](https://www.techempower.com/benchmarks)

# Features
 - CRUD API
 - database CRUD (//TODO fix docker, migrations need to start BEFORE application!)
 - ORM + custom query (Planned)
 - database migrations
 - multiple database drivers (Planned)
 - tests (Planned)
 - openapi
 - custom error handling
 - session login with oath id provider (Planned)
 - permissions (Planned)
 - github cicd

## TOConsider
 - generator for personalization of project ex:
   - removing logging
   - removing openapi
 - implementing nicer wrappers for openAPI
 - multiple log levels

# Notes
 - openAPI can be accessed by http://localhost:8080/explorer/
 - for migration creation check [sqlx-cli](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md)
   - to install `cargo install sqlx-cli`
   - to create new script `sqlx migrate add <script_name>`
 - there is no https and there will not be, you will be using proxy anyway :) [ssh proxy tutorial on intranet!](https://www.youtube.com/watch?v=qlcVx-k-02E)
