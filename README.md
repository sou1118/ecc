# ECC Client

楕円曲線暗号（ECC）をRustで実装し、Pythonから利用するためのクライアントライブラリです。

学習用として実装したため、実用性はありません。

## Prerequisites

以下のツールが必要です。

- [mise](https://github.com/jdx/mise)
- [Rust](https://www.rust-lang.org/tools/install)
- [cargo-make](https://github.com/sagiegurari/cargo-make)

### 環境設定

```bash
# Pythonのインストール
mise i

# cargo-makeのインストール（Rustがインストール済みの場合）
cargo install cargo-make
```

## セットアップ

```bash
# 開発環境のセットアップ
cargo make install-dev
```


## 使い方

```python
from client.core import ECC

# 楕円曲線の初期化
curve = ECC()

# Diffie-Hellman鍵共有
private_key, public_key = curve.generate_keypair()

```

## ディレクトリ構造

```
client/
├── src/
│   ├── client/           # Pythonソースコード
│   │   ├── core.py       # メインのPythonインターフェース
│   │   └── __init__.py
│   ├── curve.rs          # 楕円曲線の実装
│   ├── field.rs          # 有限体の実装
│   ├── lib.rs            # Rustライブラリのエントリーポイント
│   ├── point.rs          # 曲線上の点の実装
│   └── protocols/        # 暗号プロトコル実装
│       ├── diffie_hellman.rs
│       ├── elgamal.rs
│       └── mod.rs
├── stubs/                # Python型ヒント
│   └── client/
│       └── _rust.pyi
├── tests/
│   └── test_curve.py     # テストコード
├── Cargo.toml            # Rust依存関係
├── pyproject.toml        # Python設定
├── Makefile.toml         # ビルドタスク
├── mise.toml             # mise設定
└── pyrightconfig.json    # Pyright設定
```

## 開発コマンド

```bash
# テストの実行
cargo make test

# Pythonコードのフォーマット
cargo make format-python

# 型チェックとリント
cargo make check-python

# ビルド成果物のクリーンアップ
cargo make clean
```

## トラブルシューティング

### ビルドエラーが発生する場合

```bash
# 環境をクリーンアップ
cargo make clean

# 再セットアップ
cargo make install-dev
```

### Python環境の問題

```bash
# Pythonバージョンの確認
mise current python

# 必要に応じて再インストール
mise i
```
