# Hyper

Hyper is an HTML DSL focused on simplicity by reducing the inherent verbosity
that the language requires

# Goals

## CSS like syntax

```
html {
  head {
    title "Hyper!"
  }
  body {
    h1 "The Hyper DSL"
    p [className="paragraph"] {
      """Hyper is an HTML DSL focused on simplicity by reducing the inherent
      verbosity that the language requires"""
    }
  }
}
```

## Single child can be inlined

```
html {
  head {
    title "Hyper!"
  }
}
```

Maybe this too?

```
html head title "Hyper!"
```

## Tag attributes

```
p [className="paragraph"; style="display: inline"] {
  "My awesome paragraph"
}
```

## Components

```
def Avatar = [src] {
  img [src=src]
}

div {
  Avatar [src="..."]
}
```

Same name attribute

```
def Avatar = [src] {
  img [src=src] # This
}

def Avatar = [src] {
  img [src] # Can be like this
}
```

## Modules

```
# avatar.hy

pub def Avatar = [src] {
  img [src=src]
}
```

```
# index.hy

use avatar::Avatar

div {
  Avatar [src="..."]
}
```

## Control flow

### Loops

```
ul {
  for name in names {
    li name
  }
}
```

### Conditionals

```
p {
  if is_authenticated {
    "Welcome"
  } else {
    "Please, login"
  }
}
```

# License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
