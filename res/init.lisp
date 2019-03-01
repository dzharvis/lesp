(def defmacro
 (macro defmacro (name args body)
  (list (quote def) name (list (quote macro) name args body))))

(defmacro do (body...)
  (cons (quote let) (cons (list) body)))

(defmacro defn (name args body...)
 (list (quote def) name (list (quote fn) name args (cons (quote do) body))))

(defmacro when (test body...)
 (list (quote if) test (cons (quote do) body) (quote (list))))

(defmacro dbg! (args...)
 (cons (quote do) (map args (fn _ (a) (list (quote dbg) a)))))

(defn square (a)
 (* a a))

(defn cubed (a)
 (* (square a) a))

(def true (> 2 1))

(def false (not true))

(defn empty (l) (eq l (list)))

(defn not-empty (l) (not (empty l)))

(defn len (l)
 (if (empty l)
     0
     (+ 1 (len (cdr l)))))

(defn reduce_ (elems acc f)
 (if (not-empty elems)
     (reduce_ (cdr elems) (f acc (car elems)) f)
     acc))

(defn first (l) (car l))
(defn second (l) (car (cdr l)))
(defn last (l) (car (reverse l)))
(defn rest (l) (cdr l))

(defn reduce (elems arg...)
 (if (and (not-empty elems) (eq 1 (len arg)))
     (reduce_ (rest elems) (first elems) (first arg))
     (reduce_ elems (first arg) (second arg))))

(defn reverse (elems)
 (reduce_ elems (list) (fn _ (acc e) (cons e acc))))

(defn map (elems f)
 (if (not-empty elems)
     (cons (f (car elems)) (map (cdr elems) f))
     (list)))

(defn genlist (n)
 (if (> n 0)
     (cons n (genlist (- n 1)))
     (list)))
     
(defmacro ->> (forms...)
 (reduce forms (fn _ (acc form) (push acc form))))

(defmacro -> (forms...)
 (reduce forms
  (fn _ (acc form)
   (let ((fun (first form))
         (args (rest form)))
        (->> args
         (cons acc)
         (cons fun))))))
        
