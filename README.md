# oha
### oha handles anything
a language for the console

---

currently parsing
```
0 + r"hey" + 'a'
true

let a = 1 + 10
a = '\n'

let b
b = "heyy"

let c = func(a, b)
    let c = a + b

    func()
        func add(a, b) func(a, b) a + b
```

## draft

comment
```
# single line comment
##
  multiline
  comment
  with indents
```

identifiers that are nice
```
ident?
ident_
ident123
_ident
```

conditionals
```
if true
  print "hey bb"
elif false
  print "no bb"

unless false
  print "hey hey is a thing"

loop
  print("infinite loop")

while true
  print("also infinite")
```

function
```
func foo(a, b)
  pass a + b

func bar(a, b) pass b - a

func noargs() print("yo)

noargs!  # sugar for calling funcs with 0 args
noargs() # same but ugly
```

lambda
```
func(a, b) a + b
(func print "hey")! # prints "hey" .. a lambda without args?
```

`let [ident] = [expr]` -> binds variable to value
`let [ident]`      -> declares use of variable
```

let a = 1
a = r"hello \n boi"
a = "hey\n new line"

let b
b = 'c'
b = .123
```

structured data things
```
let pos

data point2
    x
    y

pos = point2(1, 2)
pos.x #=> 1
pos.y #=> 2

data point3
    x, y, z

pos = point3(1, 2, 3)
pos.x #=> 1
pos.y #=> 2
pos.z #=> 3
```

`grab [stringliteral]` -> imports oha code from given file
```
grab r"file.oha"
grab "otherfile.oha"
```
