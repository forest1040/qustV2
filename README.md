# Qust
[qulacs](https://github.com/qulacs/qulacs)の簡易版をRustに移植。
[rusq](https://github.com/hajifkd/rusq) を参考にした。

## 制限事項
- 密度行列で表現されるゲートとパウリゲートを実装
- 対応するゲートは3bitまで
- ゲートは隣接領域のみ（間接の制御ビットは制御ビットを後付けで追加する形で対応）
- 制御ビットはONの場合のみ対応
- 制御ビットの数は1bitまで（2bitまで拡張予定。。）
- 純粋状態のみ対応
- 1スレッドのCPUのみ対応

とりあえず[qulacsのチュートリアル](http://docs.qulacs.org/ja/latest/intro/4.1_python_tutorial.html)を実装できる程度をめざします。

## 対応予定
- 回転ゲート
- テストコード

