# Trunk 构建工具说明

## 配置文件 `Trunk.toml`

分层配置优先顺序 `命令行 -> 环境变量 -> 配置文件`, 命令行最优先, 其次是环境变量, 最后是配置文件

## 处理静态资源文件

要让 `Trunk` 处理静态资源, 需要满足三个要求:

- 必须声明有效的 `<link />` 标记
- 标记必须有 `data-trunk` 属性
- 必须有 `rel="{type}"` 属性, type 取值参考 [https://trunkrs.dev/assets/#copy-file](https://trunkrs.dev/assets/#copy-file)

比如拷贝文件文件的例子:

在 `index.html` 文件的 `<head/>` 区添加 `<link data-trunk rel="copy-file" href="public/data.json"/>`
然后 `trunk build` 观察 `data.json` 文件是否复制到了 `dist/` 目录

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <link data-trunk rel="copy-file" href="public/data.json"/>
    <title>Title</title>
</head>
<body>

</body>
</html>
```

在命令行中运行: 

```shell
trunk build
```

输出如下, 我们看到了 `data.json` 文件已经从 `$PROJECT_DIR/public/data.json` 复制了到 `$PROJECT_DIR/dist/data.json`.

> 不要直接把资源文件拷贝到 `dist/` 目录, 因为每次 `trunk build`都会删除 `dist/` 目录, 白辛苦!

```shell
Apr 10 21:42:30.144  INFO 📦 starting build
Apr 10 21:42:30.145  INFO spawning asset pipelines
Apr 10 21:42:30.388  INFO building yew-app
Apr 10 21:42:30.389  INFO copying file path="public/data.json"
Apr 10 21:42:30.390  INFO finished copying file path="public/data.json"
Finished dev [unoptimized + debuginfo] target(s) in 0.11s
Apr 10 21:42:30.653  INFO fetching cargo artifacts
Apr 10 21:42:30.918  INFO processing WASM
Apr 10 21:42:30.949  INFO using system installed binary app="wasm-bindgen" version="0.2.80"
Apr 10 21:42:30.949  INFO calling wasm-bindgen
Apr 10 21:42:31.200  INFO copying generated wasm-bindgen artifacts
Apr 10 21:42:31.202  INFO applying new distribution
Apr 10 21:42:31.204  INFO ✅ success
```

> 对于需要拷贝的资源文件, 建议放到一个目录当中, 复制整个目录, 而不是复制单个文件, 这样在 `index.html` 复制资源只需要一行:

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <link data-trunk rel="copy-dir" href="public"/>
    <title>Title</title>
</head>
<body>

</body>
</html>
```

> 注意 `<link/>` 标签的 `rel` 属性从 `copy-file` 改成了 `copy-dir`, 并且 `href` 属性应该是一个目录, 其路径相对于当前项目的根目录
