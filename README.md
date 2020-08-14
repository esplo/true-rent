# 実質家賃計算機

色々な条件で付与される「ぱっと見で分からない家賃以外の費用」を計算し、実質的な家賃がいくらなのかを計算するツールです。

[こちらで公開中 - 実質家賃](https://truerent.esplo.net/)

## Build

### Prerequisite

特に意味もなくwasmですので、下記のコマンドでRustとwasm-packをインストールします。

```bash
curl https://sh.rustup.rs -sSf | sh -s -- -y
source $HOME/.cargo/env
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

### For Development

開発用には下記コマンドで変更を監視します。

```bash
cd www
npm start
```

Rustに変更を加えるたび、下記コマンドでビルドします。

```bash
wasm-pack build
```

http://localhost:8080/ にアクセスすると表示されます。

### For Hosting

ビルドします。

```bash
cargo build
wasm-pack build
cd www
npm i
npm run build
```

完了すると、`www/dist`にstaticなファイル群が生成されます。
