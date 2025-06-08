# 📊 VisFile - Directory Visualization Tool

> 🚀 **「どのフォルダが容量を食っているか」が一瞬で分かる！** - 高速ディレクトリ可視化ツール

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Python](https://img.shields.io/badge/python-3670A0?style=for-the-badge&logo=python&logoColor=ffdd54)](https://www.python.org/)

## 🎯 概要

**「PCの容量不足、どこが問題？」** そんな時に威力を発揮するツールです。Rustの高速処理で巨大なディレクトリも瞬時に分析し、視覚的に分かりやすいグラフで表示します。

## 💡 こんな場面で役立ちます

- 💾 **容量不足の原因調査**: どのフォルダが容量を食っているか特定
- 🧹 **不要ファイルの発見**: 使われていない大きなファイルを発見  
- 📊 **プロジェクト分析**: コードベースの構成を視覚的に把握
- 🔍 **システム調査**: サーバーやPCのディスク使用量を調査

### ✨ 特徴

- 🚀 **爆速スキャン**: Rustによる並列処理で巨大ディレクトリも瞬時に分析
- 📊 **2つの表示方式**: 
  - **ツリーマップ**: ファイル配置の階層構造を表示
  - **バーチャート**: 使用率をパーセンテージで分かりやすく表示
- 🎯 **問題の可視化**: 容量を食っている場所が一目瞭然
- 🌍 **完全多言語対応**: 日本語・中国語・絵文字ファイル名も安全に処理
- ⚡ **1コマンド実行**: `visfile .` だけで即座に結果表示
- 🎨 **直感的なデザイン**: 色分けされた見やすいグラフ

## 🚀 今すぐ試してみる

### 30秒でインストール
```bash
git clone https://github.com/sohprivate/visfile.git
cd visfile
pip install maturin
maturin develop
./install.sh
```

### バーチャートで容量チェック（おすすめ）
```bash
# 現在のフォルダで容量の大きい順に表示
visfile . --type pie

# ホームディレクトリ全体をチェック
visfile ~ --type pie

# プロジェクトフォルダの容量内訳を表示
visfile ~/projects/my-app usage.png --type pie
```

### 🖼️ 実際の出力例

#### 📊 バーチャート - **容量問題を即座に特定！**
- **横棒の長さ** = そのフォルダの使用率
- **パーセンテージ表示** = 全体の何%を占めているか
- **実サイズ併記** = 具体的なファイルサイズ
- **上位8項目** = 容量の大きい順に自動ソート

```bash
visfile ~/Documents --type pie  # → 「Downloads が 45.2% (2.1GB) を占有」が一目で判明
```

#### 🗂️ ツリーマップ - **ファイル配置を階層表示**
```bash
visfile ~/projects/my-app        # → ディレクトリ構造を入れ子で表示
```

![例: ディレクトリ構造の可視化](test_treemap.png)

---

## 🎯 実際の使用例

### ケース1: PCの容量不足を解決
```bash
# ホームディレクトリ全体をチェック
visfile ~ home_usage.png --type pie
# → 結果: "Downloads が 40.5% (12.3GB)" と判明 → フォルダを整理して容量回復！
```

### ケース2: プロジェクトの容量分析
```bash
# Node.jsプロジェクトの分析
visfile ~/my-react-app --type pie  
# → 結果: "node_modules が 85.2% (450MB)" → 不要なパッケージを削除
```

### ケース3: サーバーのディスク調査
```bash
# ログファイルが肥大化していないかチェック
visfile /var/log --type pie
# → 結果: 古いログファイルが容量を圧迫していることが判明
```

### ケース4: 開発環境のクリーンアップ
```bash
# 各プロジェクトの容量を比較
visfile ~/projects --type pie
# → どのプロジェクトが一番容量を使っているか一目瞭然
```

---

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
※名前は `piechart` ですが、実際は見やすいバーチャート形式で出力されます。

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
A: **目的が違います！**

**📊 バーチャート（`--type pie`）- 容量問題の特定に最適**
- 「どこが一番容量を食っているか」が即座に分かる
- パーセンテージ表示で直感的
- 容量不足の原因調査に最適
- **おすすめ用途**: 容量整理、問題特定、クリーンアップ

**🗂️ ツリーマップ（デフォルト）- 構造把握に最適**  
- ファイルの配置と階層構造を表現
- 入れ子構造でプロジェクト全体を俯瞰
- **おすすめ用途**: プロジェクト分析、構造理解

### Q: 初めて使う場合はどちらがおすすめ？
A: **バーチャート（`--type pie`）がおすすめ！**
- 数値が明確で分かりやすい
- 「問題のフォルダ」がすぐ特定できる
- 実用性が高い

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