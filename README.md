# 📊 VisFile - Directory Visualization Tool

> 🚀 **高速なディレクトリ構造可視化ツール** - RustとPythonで作られたツリーマップ生成ライブラリ

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Python](https://img.shields.io/badge/python-3670A0?style=for-the-badge&logo=python&logoColor=ffdd54)](https://www.python.org/)

## 🎯 概要

VisFileは、ディレクトリ内のファイルサイズを可視化するRustライブラリです。Pythonから簡単に呼び出すことができ、ディレクトリ構造を直感的なツリーマップ画像として出力します。

### ✨ 特徴

- 🚀 **高速**: Rustによる並列処理でディレクトリスキャン
- 📊 **2つの可視化方式**: ツリーマップ（階層表示）とバーチャート（使用率表示）
- 🎯 **直感的**: パーセンテージとファイルサイズを併記
- 🐍 **使いやすい**: Pythonから1行で呼び出し可能
- 🎨 **美しい**: カラフルで見やすい画像出力
- 📦 **軽量**: 最小限の依存関係

### 🖼️ サンプル出力

#### ツリーマップ（階層表示）
```python
import visfile
visfile.treemap("my_project/", "project_map.png")
```

#### バーチャート（使用率表示）
```python
import visfile
visfile.piechart("my_project/", "usage_chart.png")
```

![例: ディレクトリ構造の可視化](test_treemap.png)

## 🚀 クイックスタート

### 必要条件

- Rust (最新安定版)
- Python 3.8+
- maturin

### 📦 インストール（初回のみ）

```bash
# 1. リポジトリをクローン
git clone https://github.com/sohprivate/visfile.git
cd visfile

# 2. maturinをインストール
pip install maturin

# 3. ライブラリをビルド＆インストール
maturin develop

# 4. CLIコマンドをインストール（推奨）
./install.sh
```

### 🌍 どこからでも使用可能

インストール後は、**任意のディレクトリ**から`visfile`コマンドが使用できます！

```bash
# CLIコマンドで実行（install.sh実行後）
visfile .                           # ツリーマップ（デフォルト）
visfile . --type pie                # バーチャート（使用率表示）
visfile ~/Documents docs.png        # Documentsフォルダを指定ファイル名で
visfile /path/to/project chart.png -t pie  # バーチャート形式で出力

# ヘルプを表示
visfile --help
```

**✅ git cloneしたフォルダの中にいる必要はありません！**

### 使用方法

#### 基本的な使い方

```python
import visfile

# 現在のディレクトリを可視化
visfile.treemap(".", "my_directory.png")

# 特定のプロジェクトを可視化
visfile.treemap("/path/to/project", "project_visualization.png")
```

#### 🖥️ CLIコマンドで実行（推奨）

```bash
# install.sh実行後、グローバルコマンドとして使用可能

# ツリーマップ（階層表示）
visfile .                           # 現在のディレクトリ -> treemap.png
visfile /path/to/project            # 指定ディレクトリ -> treemap.png
visfile src/ source_map.png         # カスタム出力ファイル名

# バーチャート（使用率表示）
visfile . --type pie                # 現在のディレクトリ -> バーチャート
visfile ~/Documents docs_chart.png -t pie  # 使用率チャート
visfile src/ usage.png --type pie   # ソースコードの使用率

# 実用例
visfile ~/Documents                 # ツリーマップでDocuments可視化
visfile ~/Documents --type pie      # バーチャートでDocuments使用率表示
visfile ~/Desktop/project chart.png -t pie  # プロジェクト使用率

# ヘルプとバージョン
visfile --help                      # 使用方法を表示
visfile --version                   # バージョン情報
```

#### 📝 スクリプトで直接実行（代替方法）

```bash
# install.shを使わない場合
python visfile_cli.py .                    # ツリーマップ
python visfile_cli.py . --type pie         # バーチャート
python visfile_cli.py ~/Documents docs.png -t pie  # 使用率チャート
```

#### 🐍 Pythonから実行

```bash
# 1行で実行
python -c "import visfile; visfile.treemap('.', 'quick_view.png')"

# テストスクリプトを実行
python test_visfile.py
```

## 📖 詳細ガイド

### API リファレンス

#### `visfile.treemap(path, output)`

ディレクトリ構造をツリーマップ画像として可視化します（階層表示）。

**パラメータ:**
- `path` (str): 分析するディレクトリのパス  
- `output` (str): 出力PNG画像のファイルパス

#### `visfile.piechart(path, output)`

ディレクトリ使用率をバーチャート画像として可視化します（パーセンテージ表示）。

**パラメータ:**
- `path` (str): 分析するディレクトリのパス
- `output` (str): 出力PNG画像のファイルパス

**例:**

```python
import visfile

# ツリーマップ（階層表示）
visfile.treemap("~/Documents", "documents_map.png")
visfile.treemap("/usr/local", "system_map.png")
visfile.treemap("src/", "source_code_map.png")

# バーチャート（使用率表示）
visfile.piechart("~/Documents", "documents_usage.png")
visfile.piechart("/usr/local", "system_usage.png")
visfile.piechart("src/", "source_usage.png")

# エラーハンドリング
try:
    visfile.treemap("non_existent_path", "output.png")
except Exception as e:
    print(f"エラー: {e}")
```

### 🎨 出力画像の見方

#### ツリーマップ
- **四角の大きさ** = ファイル/ディレクトリのサイズに比例
- **色の違い** = ディレクトリの階層の深さ
- **ラベル** = ファイル名とフォーマットされたサイズ表示

#### バーチャート
- **バーの長さ** = ディレクトリの使用率（パーセンテージ）
- **色分け** = 各ディレクトリを識別しやすく
- **ラベル** = ディレクトリ名、パーセンテージ、実際のファイルサイズ
- **表示** = 上位8項目を使用率順に表示

### 🏗️ プロジェクト構造

```
visfile/
├── src/
│   └── lib.rs          # メインRustライブラリ
├── Cargo.toml          # Rust依存関係設定
├── pyproject.toml      # Python/maturinビルド設定
├── visfile_cli.py      # CLIインターフェース
├── install.sh          # CLIインストールスクリプト
├── test_visfile.py     # テストスクリプト
└── README.md           # このファイル
```

## 🔧 開発者向け情報

### 使用技術

- **[walkdir](https://crates.io/crates/walkdir)**: 高速ディレクトリ探索
- **[rayon](https://crates.io/crates/rayon)**: 並列処理
- **[plotters](https://crates.io/crates/plotters)**: PNG画像生成
- **[PyO3](https://crates.io/crates/pyo3)**: Python-Rustバインディング
- **[maturin](https://github.com/PyO3/maturin)**: Python拡張モジュールビルド

### ビルドの詳細

```bash
# 開発用ビルド
maturin develop

# リリース用ビルド
maturin build --release

# テスト実行
python test_visfile.py
```

### アーキテクチャ

1. **ディレクトリスキャン**: `walkdir`で再帰的にファイルを探索
2. **データ構造構築**: ツリー構造でディレクトリ階層を表現
3. **サイズ計算**: 各ディレクトリの合計サイズを計算
4. **レイアウト生成**: ツリーマップアルゴリズムで座標を計算
5. **画像描画**: `plotters`でPNG画像を生成

## 🤝 コントリビュート

1. このリポジトリをフォーク
2. フィーチャーブランチを作成 (`git checkout -b feature/amazing-feature`)
3. 変更をコミット (`git commit -m 'Add amazing feature'`)
4. ブランチをプッシュ (`git push origin feature/amazing-feature`)
5. プルリクエストを開く

## 📄 ライセンス

このプロジェクトはMITライセンスの下で公開されています。

## 🙋‍♂️ よくある質問

### Q: 大きなディレクトリでも動作しますか？
A: はい。Rustの並列処理により、大容量ディレクトリも高速に処理できます。

### Q: 2つの可視化方式の違いは？
A: **ツリーマップ**は階層構造を重視し、**バーチャート**は使用率を重視します。
- ツリーマップ: 入れ子構造でファイル配置を表現
- バーチャート: パーセンテージで使用率を直感的に表示

### Q: どちらの方式を使うべき？
A: 目的によって使い分けてください：
- **ディレクトリ構造を把握したい** → ツリーマップ
- **容量を多く使っているフォルダを特定したい** → バーチャート

### Q: 出力画像のサイズは変更できますか？
A: 現在は1200x800ピクセル固定ですが、今後のバージョンで設定可能にする予定です。

### Q: エラーが発生したらどうすれば？
A: まず`python test_visfile.py`を実行してテストが通るかご確認ください。

### Q: どこからでもCLIコマンドを使えますか？
A: **はい！** `maturin develop`でインストール後、システム全体で`visfile`コマンドが使用可能です。git cloneしたフォルダにいる必要はありません。

### Q: インストール後の使い方は？
A: 4ステップで完了：
1. `git clone` & `maturin develop` & `./install.sh`（初回のみ）
2. 任意の場所に`cd`で移動
3. `visfile .`（ツリーマップ）または `visfile . --type pie`（バーチャート）

---

<div align="center">

**⭐ このプロジェクトが役立ったら、ぜひスターをお願いします！ ⭐**

[🐛 バグ報告](https://github.com/sohprivate/visfile/issues) • [💡 機能要望](https://github.com/sohprivate/visfile/issues) • [📧 お問い合わせ](https://github.com/sohprivate)

</div>