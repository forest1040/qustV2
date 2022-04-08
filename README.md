# Qust
[qulacs](https://github.com/qulacs/qulacs)の簡易版をRustに移植。
[rusq](https://github.com/hajifkd/rusq) を参考にした。

- 密度行列で表現されるゲートとパウリゲートを実装
- 対応するゲートは3bitまで
- ゲートは隣接領域のみ（間接の制御ビットは制御ビットを後付けで追加する形で対応）
- 制御ビットはONの場合のみ対応
- 制御ビットの数は2bitまで
- 純粋状態のみ対応
- 1スレッドのCPUのみ対応
