# nosql-workbench-converter a.k.a. nwc
[NoSQLWorkbench](https://docs.aws.amazon.com/ja_jp/amazondynamodb/latest/developerguide/workbench.html)が出力するjsonをCloudFormationのjsonに変換します。

```zsh
cat hoge.json | jq -c | nwc > hoge.json
```
