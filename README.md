# 📊 VisFile - Directory Visualization Tool

> 🚀 **高速なディレクトリ構造可視化ツール** - RustとPythonで作られたツリーマップ生成ライブラリ

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Python](https://img.shields.io/badge/python-3670A0?style=for-the-badge&logo=python&logoColor=ffdd54)](https://www.python.org/)

## 🎯 概要

VisFileは、ディレクトリ内のファイルサイズを可視化するRustライブラリです。Pythonから簡単に呼び出すことができ、ディレクトリ構造を直感的なツリーマップ画像として出力します。

### ✨ 特徴

- 🚀 **高速**: Rustによる並列処理でディレクトリスキャン
- 📊 **直感的**: ファイルサイズに比例したツリーマップ表示
- 🐍 **使いやすい**: Pythonから1行で呼び出し可能
- 🎨 **美しい**: 階層ごとに色分けされた見やすい画像出力
- 📦 **軽量**: 最小限の依存関係

### 🖼️ サンプル出力

```python
import visfile
visfile.treemap("my_project/", "project_map.png")
```

![例: ディレクトリ構造のツリーマップ](test_treemap.png)

## 🚀 クイックスタート

### 必要条件

- Rust (最新安定版)
- Python 3.8+
- maturin

### インストール

```bash
# 1. リポジトリをクローン
git clone https://github.com/sohprivate/visfile.git
cd visfile

# 2. maturinをインストール
pip install maturin

# 3. ライブラリをビルド＆インストール
maturin develop
```

### 使用方法

#### 基本的な使い方

```python
import visfile

# 現在のディレクトリを可視化
visfile.treemap(".", "my_directory.png")

# 特定のプロジェクトを可視化
visfile.treemap("/path/to/project", "project_visualization.png")
```

#### コマンドラインから実行

```bash
# 1行で実行
python -c "import visfile; visfile.treemap('.', 'quick_view.png')"

# テストスクリプトを実行
python test_visfile.py
```

## 📖 詳細ガイド

### API リファレンス

#### `visfile.treemap(path, output)`

ディレクトリ構造をツリーマップ画像として可視化します。

**パラメータ:**
- `path` (str): 分析するディレクトリのパス
- `output` (str): 出力PNG画像のファイルパス

**例:**

```python
import visfile

# 様々な使用例
visfile.treemap("~/Documents", "documents_map.png")
visfile.treemap("/usr/local", "system_map.png")
visfile.treemap("src/", "source_code_map.png")

# エラーハンドリング
try:
    visfile.treemap("non_existent_path", "output.png")
except Exception as e:
    print(f"エラー: {e}")
```

### 🎨 出力画像の見方

- **四角の大きさ** = ファイル/ディレクトリのサイズ
- **色の違い** = ディレクトリの階層の深さ
- **ラベル** = ファイル名とフォーマットされたサイズ表示

### 🏗️ プロジェクト構造

```
visfile/
├── src/
│   └── lib.rs          # メインRustライブラリ
├── Cargo.toml          # Rust依存関係設定
├── pyproject.toml      # Python/maturinビルド設定
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

### Q: 出力画像のサイズは変更できますか？
A: 現在は1200x800ピクセル固定ですが、今後のバージョンで設定可能にする予定です。

### Q: エラーが発生したらどうすれば？
A: まず`python test_visfile.py`を実行してテストが通るかご確認ください。

---

<div align="center">

**⭐ このプロジェクトが役立ったら、ぜひスターをお願いします！ ⭐**

[🐛 バグ報告](https://github.com/sohprivate/visfile/issues) • [💡 機能要望](https://github.com/sohprivate/visfile/issues) • [📧 お問い合わせ](https://github.com/sohprivate)

</div>