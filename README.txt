cargo build

输入格式：./target/debug/myfind <目录> --name <正则表达式> (-v)
目录 / 正则表达式可多个


测试用例：
./target/debug/myfind ./test/a ./test/b --name b

./target/debug/myfind ./test/a ./test/b --name b 1

./target/debug/myfind ./test/a ./test/b --name b 1 -v

输入格式错误用例：
./target/debug/myfind ./test/a ./test/c b 1 -v

./target/debug/myfind ./test/a ./test/c --name b 1 -v
