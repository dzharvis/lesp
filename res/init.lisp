(def defmacro
 (macro defmacro (name args body)
  (list (quote def) name (list (quote macro) name args body))))

(defmacro defn (name args body)
 (list (quote def) name (list (quote fn) name args body)))

(defmacro when (test body)
 (list (quote if) test body (quote (list))))

(defn square (a)
 (* a a))

(defn cubed (a)
 (* (square a) a))

(def true (> 2 1))

(def false (not true))

(defn empty (l) (eq l (list)))

(defn not-empty (l) (not (empty l)))

(defn reduce (elems acc f)
 (if (not-empty elems)
     (reduce (cdr elems) (f acc (car elems)) f)
     acc))

(defn reduce- (elems f)
 (if (not-empty elems)
     (reduce (cdr elems) (car elems) f)
     acc))

(defn map (elems f)
 (if (not-empty elems)
     (cons (f (car elems)) (map (cdr elems) f))
     (list)))

(defn reverse (elems)
 (reduce elems (list) (fn _ (acc e) (cons e acc))))

(defn genlist (n)
 (if (> n 0)
     (cons n (genlist (- n 1)))
     (list)))