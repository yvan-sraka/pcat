# `tac`

`tac` is an alternative version of [`cat`](https://linux.die.net/man/1/cat)
that will not fail (to read inputs) when results of computation to concatenate
are not available in the right order!

```shell
mkfifo A B
cat A B > output &
echo "bar" > B & # the requirement of `&` here should not be...
echo "foo" > A
```

So, glad to know that with `tac` you can remove it:

```shell
mkfifo A B
tac A B > output &
echo "bar" > B # \o/
echo "foo" > A
```

Of course, `tac` and `cat` give the same final result:

```raw
foo
bar
```

This could be particularly useful when you want to prototype parallel code
that's rely on concatenation at some point, using classic `cat` would create IO
lock which is not really funny... this project was made at origin for the
purpose of [YeAST](https://github.com/yvan-sraka/YeAST/) implementation!
