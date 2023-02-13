[Zero To Production In Rust](https://www.lpalmieri.com/)を進めています。
以下はそのメモです。

## 実現したいユーザストーリー
* ブログの訪問者として、ニュースレターを購読して、ブログに新しいコンテンツが公開されたときに最新情報を電子メールで受信できるようにしたいと考えています。
* ブログの作成者として、新しいコンテンツが公開されたときに通知できるように、すべての購読者に電子メールを送信したいと考えています。
* 購読者として、ニュースレターの購読を解除して、ブログからの更新メールの受信を停止できるようにしたいと考えています。

## ディレクトリ構成
ディレクトリ構成は以下の通りです。

```
.
├── .github
│   └── workflows
│       └── ci.yml // プルリク時のワークフロー
├── readme.md  // 本ファイル
└── server // APIサーバのフォルダ
    ├── .gitignore
    ├── Cargo.lock
    ├── Cargo.toml
    ├── src
    │   ├── lib.rs
    │   └── main.rs
    └── tests
        └── health_check.rs
```

## 利用するミドルウェア
* Webフレームワーク：actix_web
* DBフレームワーク：sqlx

## 利用するCIツール
* テスト：cargo test
* コードカバレッジ：cargo llvm-cov
* リンティング：cargo clippy
* フォーマット：cargo fmt
* 脆弱性対策：cargo audit
* パイプライン:：Github Actions

## 各記事に対する備忘
### #1 Setup - Toolchain, IDEs, CI
* IDEにはvscodeを使用する
* コードカバレッジにはcargo llvm-covを使用する
* cargo auditは予めopenssl関連のライブラリがインストールされていないとビルドエラーになる

### #2 Learn By Building An Email Newsletter

### #3 How To Bootstrap A Rust Web API From Scratch
* tokio::spwnクレートからmainを呼び出すことによりアプリケーションをバックグラウンドタスクとして実行する
* 非同期部分のロジックが分からないため、tokioを利用したasync/awaitの仕組みについてはちゃんと学習したい
* アプリケーションポートは実行時に決定されるため引数として与えて制御することはできない。
TcpListnerを利用して空いているポートをOSによりバインドしたlistnerを作成し、HttpServerに引き渡すことで空いているポートを利用して通信ができる。
* health_check.rsにおいて、構造体としてAppを作成しfutureタスクをテストスクリプト内で作成、dropすることで、明示的なfutureタスクのdropを行った。

### #3.5 HTML forms, Databases, Integration tests
* web::Form<構造体>とすることで、構造体のフィールド名とbodyの変数が対応する値を簡単に設定することができる
* serdeを使用することで構造化データ（JSONやYAMLなど）のシリアライズ／デシリアライズすることができる。
* 以下の文章はちゃんと理解できていないため後で復習したい。
> We now have a good picture of what is happening:
> * before calling subscribe actix-web invokes the from_request method for all subscribe's input arguments: in our case, Form::from_request;
> * Form::from_request tries to deserialise the body into FormData according to the rules of URL-encoding leveraging serde_urlencoded and the Deserialize implementation of FormData, automatically generated for us by #[derive(serde::Deserialize)];
> * if Form::from_request fails, a 400 BAD REQUEST is returned to the caller. If it succeeds, subscribe is invoked and we return a 200 OK.
* Postgreのみを対象にsqlx-cliをインストールしたい場合は[公式ドキュメント](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli#with-rust-toolchain)を参考に以下のコマンドを入力する。
``` bash
# only for postgres
$ cargo install --version=0.6.0 sqlx-cli --no-default-features --features native-tls,postgres
```
* ローカルのpgadminから構築したpostgresコンテナに接続する方法が分からなかった。postgresコンテナにpgadminを入れる必要があるのは分かるが、手順が不明である。


## 参考資料

### CI構築
* [Setup - Toolchain, IDEs, CI](https://www.lpalmieri.com/posts/2020-06-06-zero-to-production-1-setup-toolchain-ides-ci/)
* [100日後にRustをちょっと知ってる人になる: [Day 72]脆弱性チェック: cargo audit](https://zenn.dev/shinyay/articles/hello-rust-day072)
* [Rustの新しいコードカバレッジ/Source-based code coverage](https://qiita.com/dalance/items/69e18fe300760f8d7de0)
* [GitHubの新機能「GitHub Actions」で試すCI/CD](https://knowledge.sakura.ad.jp/23478/?gclid=CjwKCAiAuaKfBhBtEiwAht6H75-E5CRDd-qy1ZLk2Bxcmj1uDFsn9BgGU4EHjGdc1nWUP_NxJXdacxoCrtEQAvD_BwE)
* [Rustとgithub actionsでCI環境構築](https://zenn.dev/naokifujita/articles/c890954165c21f)

### 非同期処理
* [Asynchronous Programming in Rust](https://async-book-ja.netlify.app/01_getting_started/01_chapter.html)
* [Tokio チュートリアル (日本語訳)](https://zenn.dev/magurotuna/books/tokio-tutorial-ja)

### シリアライズ／デシリアライズ
* [RustのSerdeの簡単な紹介](https://qiita.com/garkimasera/items/0442ee896403c6b78fb2)

### データベース周り
* [launchbadge/sqlx](https://github.com/launchbadge/sqlx)
* [launchbadge/sqlx/sqlx-cli](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli#with-rust-toolchain)

### その他
* [ASCII Encoding Reference](https://www.w3schools.com/tags/ref_urlencode.ASP)