(def square
 (fn square (a)
  (* a a)))

(def cubed
 (fn cubed (a)
  (* (square a) a)))

(def true (> 2 1))

(def false (not true))

(def empty (fn empty (l) (eq l (list))))

(def not-empty (fn not-empty (l) (not (empty l))))

(def reduce
 (fn reduce (elems acc f)
    (if (not-empty elems)
        (reduce (cdr elems) (f acc (car elems)) f)
        acc)))

(def map
 (fn map (elems f)
    (if (not-empty elems)
        (cons (f (car elems)) (map (cdr elems) f))
        (list))))

