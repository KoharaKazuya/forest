# forest

[![Build Status](https://travis-ci.org/KoharaKazuya/forest.svg?branch=master)](https://travis-ci.org/KoharaKazuya/forest)

forest は行指向で構造化された木構造を表すテキストを整形して出力するコマンド。


## 例

(sample1.txt)

```
pattern1_root
pattern1_root/pattern2_last_leaf
root_node
root_node/pattern3_non-last_leaf
root_node/pattern3_non-last_leaf/sample
root_node/pattern3_non-last_leaf/pattern4_non-last_node's_child
root_node/leaf_node
```

```console
$ cat sample1.txt | forest
├ ─ pattern1_root
│   └ ─ /pattern2_last_leaf
└ ─ root_node
    ├ ─ /pattern3_non-last_leaf
    │   ├ ─ /sample
    │   └ ─ /pattern4_non-last_node's_child
    └ ─ /leaf_node
```

(sample2.txt)

```
サ
サン
サンプル
forest
forest doesn't specify any characters
forest doesn't specify any characters as separater.
```

```console
$ cat sample2.txt | forest
├ ─ サ
│   └ ─ ン
│       └ ─ プル
└ ─ forest
    └ ─  doesn't specify any characters
        └ ─  as separator.
```


## アルゴリズム

アルゴリズムを擬似コードで示す。

```
入力: テキスト input
出力: テキスト output

entries ← 空キュー

// 木構造構築フェーズ
ancestors ← 空スタック
prev ← null
while line ← input の先頭行
  while ancestor ← ancestors.pop()
    if line が ancestor.value から始まる
      ancestors.push(ancestor)
      break
    if prev != null
      prev.shape ← CLOSE
    prev ← ancestor
  entry ← (depth: ancestors の長さ + 1, shape: Pending, value: line)
  if prev != null
    prev.shape ← OPEN
  ancestors.push(entry)
  entries.enqueue(entry)
while ancestor ← ancestors.pop()
  ancestor.shape ← CLOSE

// 木構造出力フェーズ
ancestor_shapes ← 空リスト
while entry ← entries.dequeue()
  ancestor_shapes ← ancestor_shapes の先頭 (entry.depth - 1) 要素
  shape for each ancestor_shapes
    if shape = OPEN
      ANCESTOR_GUIDE_OPEN を output 出力
    else if shape = CLOSE
      ANCESTOR_GUIDE_CLOSE を output 出力
  if entry.shape = OPEN
    NAME_GUIDE_OPEN を output に出力
  else if entry.shape = CLOSE
    NAME_GUIDE_CLOSE を output に出力
  entry.value を output 出力
  ancestor_shapes の末尾に entry.shape を追加
```
