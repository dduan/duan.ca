# When Default Parameter Values Fall Short In Swift
2015-04-18T00:00:00-08:00
tag: Swift

Swift supports default value for function parameters:

```
    func f(x=1, y=2, z=3) { return x * y + z }

```
Functions defined in this way is extremely flexible as they allows the user to arbitrarily omit any parameters:

```
    f(y:6) // 9

```
Today, however, I ran into a scenario where default values stopped working.

imagine a group of functions who share the same implementation function by supplying different values as its parameters. Furthermore, each of these functions also expose a same set of parameters.

```
    func impl(x:String, a:Int, b:Int, c:Int, d:Int, f:Int, e:Int)

```
