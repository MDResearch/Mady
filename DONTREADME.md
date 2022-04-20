# Internal Only

[![Rust Test](https://github.com/MDResearch/research/actions/workflows/rust.yml/badge.svg)](https://github.com/MDResearch/research/actions/workflows/rust.yml) [![Project](https://img.shields.io/badge/Project-WIP-brightgreen)](https://github.com/orgs/MDResearch/projects/3)

## Branch

- main
  - 不要傳會 action test fail 的程式碼
  - 照 [Project](https://github.com/orgs/MDResearch/projects/3) 分工

## 基本設置

- 環境
  - [Rust 2021](https://www.rust-lang.org/tools/install)
  - [Git](https://git-scm.com/)
  - [Just](https://just.systems/)
- 工具
  - [VSCode](https://code.visualstudio.com)
  - [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer)
  - [GitLens — Git supercharged](https://marketplace.visualstudio.com/items?itemName=eamodio.gitlens)
  - [Better Comments](https://marketplace.visualstudio.com/items?itemName=aaron-bond.better-comments)
  - [Error Lens](https://marketplace.visualstudio.com/items?itemName=usernamehw.errorlens)

## 基本架構

- mady_math
  放自動微分所需要的的數學函式及數學結構，像是`Tensor`、`Matrix`、`Max`、`Min`、`Sigmoid`、`Relu`、`He`、`GradAdd` 等等
- mady_macro
  放解析自動微分 macro 的地方，有 `#[grad]`
- src
  重定向用 (reexport)

## 基本原則

當有新的程式碼要`PUSH`先確認過以下幾點

1. `just a` 通過 (其他指令可以用 `just -l` 查看)
2. 確認並更改 [Project](https://github.com/orgs/MDResearch/projects/3)
3. 有 `breakthrough change` 請另行討論

## 油圖

上傳程式碼前記得看一下油圖，養護身心靈

[![油圖](https://pixiv.cat/83554234-2.png)](https://www.pixiv.net/artworks/83554234)
