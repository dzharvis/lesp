(def square
 (fn square (a)
  (* a a)))

(def cubed
 (fn cubed (a)
  (* (square a) a)))
