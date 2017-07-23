# forest

[![Build Status](https://travis-ci.org/KoharaKazuya/forest.svg?branch=master)](https://travis-ci.org/KoharaKazuya/forest)


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
