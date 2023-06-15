---
marp: true
header: さうすの Rust 勉強会::lesson05
class: invert
paginate: true
---

# さうすの Rust 勉強会

## lesson05

---

## 内容

- 応用プログラムを作る
  - ログイン付きのファイルアップロード・共有ソフトウェア
    - プライベート・制限付き共有・公開
  - 今回に限ってフロントエンドも作る

---

## フロントエンド

今回は Vite を使います。

```shell
npm create vite@latest frontend --template react-ts
```

---

## Session

Session というのは、

- サーバー側に保存された、ブラウザに紐付けられた情報
  - よってユーザーはその内容を読めないし、書けない（「削除」することはできる）
- ブラウザに紐付けるため、Cookies に Session ID が入っている
  - ユーザーがそれを削除すると、紐付きができなくなる
- 認証と関係ある情報を Session に保存することはよくある

https://crates.io/crates/axum-sessions を用いて実装する。

---

## `async_graphql::Guard`

---

## Authentication（認証） v.s. Authorization（認可）

- Authentication
  - 本人確認
  - 今回は session を用いて実装する
- Authorization
  - 権限
  - 今回では guard を用いて実装する

---

## S3 / Minio + Presigned URL

---

## Containerize（Container 化）

アプリを Docker 化すると、簡単にいろんなサーバーにデプロイできる。Kubernetes などの環境にも動かせることができる。

---

## デプロイ

---

## 課題

- `fileshare` のソースコードを読んで、理解せよ
  - わからないことあったら Slack で聞こう
