# poxter

rust で twitter のクローンアプリを作る

## ディレクトリ

```
.
├── .env            // 各環境変数をまとめている
├── Cargo.toml      // src配下のクレートをワークスペースとしてまとめている
├── README.md
├── docker          // Docker環境
│   └── local       // local環境
├── generate.sh     // データベースからエンティティを作成するためのシェルスクリプト
├── server          // main関数を実装しているクレート
├── api             // APIのurlを実装しているクレート
├── usecase         // ユースケースを実装しているクレート
├── domain          // ドメインを実装しているクレート
├── infrastructure  // DBとの接続を実装しているクレート
├── entity          // エンティティを実装しているクレート(DBから作られる)
├── migration       // DBのマイグレーションを実装しているクレート
└── tests           // 結合テストを実装しているクレート
    └── run_test.rs // 結合テスト
```

### 利用技術

- [actix-web](https://github.com/actix/actix-web)
- [SeaORM](https://github.com/SeaQL/sea-orm)

