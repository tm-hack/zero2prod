# プロジェクトの概要
[Zero To Production In Rust](https://www.lpalmieri.com/posts/)を進めています。

以下はそのメモです。

## 実現したいユーザストーリー
* ブログの訪問者として、ニュースレターを購読して、ブログに新しいコンテンツが公開されたときに最新情報を電子メールで受信できるようにしたいと考えています。
* ブログの作成者として、新しいコンテンツが公開されたときに通知できるように、すべての購読者に電子メールを送信したいと考えています。
* 購読者として、ニュースレターの購読を解除して、ブログからの更新メールの受信を停止できるようにしたいと考えています。

## 利用するミドルウェア
* Webフレームワーク：actix_web

## 利用するCIツール
* テスト：cargo test
* コードカバレッジ：cargo llvm-cov
* リンティング：cargo clippy
* フォーマット：cargo fmt
* 脆弱性対策：cargo audit
* パイプライン:：Github Actions（予定）

## 参考資料

### CIツール
[Setup - Toolchain, IDEs, CI](https://www.lpalmieri.com/posts/2020-06-06-zero-to-production-1-setup-toolchain-ides-ci/)

[100日後にRustをちょっと知ってる人になる: [Day 72]脆弱性チェック: cargo audit](https://zenn.dev/shinyay/articles/hello-rust-day072)

[Rustの新しいコードカバレッジ/Source-based code coverage](https://qiita.com/dalance/items/69e18fe300760f8d7de0)
