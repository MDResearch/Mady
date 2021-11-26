# Research

[![Rust Test](https://github.com/MDResearch/research/actions/workflows/rust.yml/badge.svg)](https://github.com/MDResearch/research/actions/workflows/rust.yml)[![Project](https://img.shields.io/badge/Project-WIP-brightgreen)](https://github.com/orgs/MDResearch/projects/2)

**不要傳會action test fail的程式碼**

## 基本設置

- 環境
    - [Rust 2021](https://www.rust-lang.org/tools/install)
    - [Git](https://git-scm.com/)
- 工具
    - [VSCode](https://code.visualstudio.com)
    - [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer)
    - [GitLens — Git supercharged](https://marketplace.visualstudio.com/items?itemName=eamodio.gitlens)

## 基本架構

- ad
    放自動微分所需要的的數學函式及數學結構，像是`Tensor`、`Matrix`、`Max`、`Min`、`Sigmoid`、`Relu`、`He` 等等
- mad
    放解析自動微分macro的地方，有 `#[ad]`
- src
    主要執行程式

## 基本原則

當有新的程式碼要`PUSH`先確認過以下幾點
1. `cargo test --all` 通過
2. `cargo fmt` 整理程式碼
3. 確認並更改 `Project`
4. 有 `breakthrough change` 請另行討論


## 油圖

上傳程式碼前記得看一下油圖，養護身心靈

[![油圖](https://pixiv.cat/83554234-2.png)](https://www.pixiv.net/artworks/83554234)