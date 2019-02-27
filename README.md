# Much worse version of Lisp

Try it here https://dzharvis.github.io/lesp/

## Building
### Local console app
 - `cargo run --no-default-features`
### Web app
 - `cargo install cargo-web`
 - `cargo web start`
 - Open `http://[::1]:8000` in your browser

## Usage example
```lisp
>> (reduce (list 1 2 3) 0 +)
<< 6

>> (map (list 1 2 3) square)
<< [1, 4, 9]

>> (defn add (a b) (+ a b))
<< "add"

>> (add 10 20)
<< 30
```

## Notes
 - Empty list works as nil
 - Everything is immutable