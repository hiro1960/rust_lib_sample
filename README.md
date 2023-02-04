独立した、ライブラリcrateを作成し、それを使用するサンプル

cargoを使って、プロジェクトの依存関係を管理する。
ます、以下のコマンドで、ライブラリcrateであるbar、およびそれを使用するfooを作成する

# A binary
> cargo new foo

# A library
> cargo new --lib bar

barにはデフォルトでadd(x,y)関数が作られる。これを、fooからcallしてみる。

fooのCargo.tomlのdependeciesに以下を追加して、ライブラリbarを参照できる様にする。

[dependencies]
bar = { path =  "../bar" }  # ローカルのファイルシステムのパスを指定

これで、fooのmain.rsでは、use bar;にてライブラリbarを参照できるようになる。
（modは、同一crate内のモジュールを参照する時に使用する。今回は他crateのモジュールを使用するので、useを使う。）