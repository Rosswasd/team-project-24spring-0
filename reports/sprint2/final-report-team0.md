# Moras - Sprint2

## 1. Metrics

## 2. Documentation
Documentation for end users: []()  
Documentation for developers: [https://sustech-cs304.github.io/team-project-24spring-0/moras/](https://sustech-cs304.github.io/team-project-24spring-0/moras/)  

## 3. Tests
- tools: `cargo tarpaulin`  
- tasks: `cargo tarpaulin --out html`  
- source code: [tests folder](../../src-tauri/src/tests)  
- coverage report:  
  ![coverage report](img/coverage.png)

## 4. Build
- tools: `cargo`, `npm`, `github CD`  
- frameworks: `tauri`  
- tasks: `cargo tauri build`  
- executable: []()  
- buildfile: [Cargo.toml](../../src-tauri/Cargo.toml), [package.json](../../src-ui/package.json), [release.yaml](../../.github/workflows/release.yml)  

## 5. Deployment
This is a desktop application without any online services, so there is no need for deployment. However, we use GitHub CD to automatically build our application for multiple platforms. [release.yaml](../../.github/workflows/release.yml)  