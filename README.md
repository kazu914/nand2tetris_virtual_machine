# これは何
[コンピュータシステムの理論と実装 ―モダンなコンピュータの作り方](https://www.amazon.co.jp/%E3%82%B3%E3%83%B3%E3%83%94%E3%83%A5%E3%83%BC%E3%82%BF%E3%82%B7%E3%82%B9%E3%83%86%E3%83%A0%E3%81%AE%E7%90%86%E8%AB%96%E3%81%A8%E5%AE%9F%E8%A3%85-%E2%80%95%E3%83%A2%E3%83%80%E3%83%B3%E3%81%AA%E3%82%B3%E3%83%B3%E3%83%94%E3%83%A5%E3%83%BC%E3%82%BF%E3%81%AE%E4%BD%9C%E3%82%8A%E6%96%B9-Noam-Nisan/dp/4873117127)(通称nand2tetris)のバーチャルマシン実装.  
第7,8章に該当する.
`rust`で書いてる．  

# テストプログラムの通過状況(書籍準拠)
## 7章

|通ったか|機能|項目名|
|:-:|:-:|:-:|
|:heavy_check_mark:|スタック算術|SimpleAdd|
|:heavy_check_mark:|スタック算術|StackTest|
|:heavy_check_mark:|メモリアクセス|BasicTest|
|:heavy_check_mark:|メモリアクセス|PointerTest|
|:heavy_check_mark:|メモリアクセス|StaticTest|

## 8章

|通ったか|機能|項目名|
|:-:|:-:|:-:|
|:heavy_check_mark:|プログラムフロー|BasicLoop|
|:heavy_check_mark:|プログラムフロー|Fibonacci|
|:black_square_button:|関数呼び出し|SimpleFunction|
|:black_square_button:|関数呼び出し|FibonacciElement|
|:black_square_button:|関数呼び出し|StaticsTest|
