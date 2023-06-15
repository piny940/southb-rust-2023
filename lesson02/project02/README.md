# lesson02::project02

これは、ファイルシステムを用いた key-value database である。実装していない部分があるので、実装しよう。

- 今回はファイルシステムを使うので、サーバーが再起動されても保存された値はあるはず。
- ファイルシステムの使い方については [std::fs::read_to_string](https://doc.rust-lang.org/std/fs/fn.read_to_string.html)、および [std::fs::write](https://doc.rust-lang.org/std/fs/fn.write.html) のドキュメントを確認せよ。
- 保存先ですが、このディレクトリの `/files` ディレクトリを使おう。キーは有効なファイル名のみ処理する必要があるが、有効でないファイル名の処理できればなおさらよい。

## やり方

- `src/main.rs` で `todo!()` になっているところを埋めよう
- `cargo test` を実行して、テストが通っていることを確認しよう
