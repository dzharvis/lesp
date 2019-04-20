(def defmacro
 (macro defmacro (name args body)
  (list (quote def) name (list (quote macro) name args body))))

(def nil (list))

(defmacro comment (forms...)
 (quote nil))

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
(defn rest (l) (cdr l))
(defn rrest (l) (rest (rest l)))

(defn reduce (elems arg...)
 (if (and (not-empty elems) (eq 1 (len arg)))
     (reduce_ (rest elems) (first elems) (first arg))
     (reduce_ elems (first arg) (second arg))))

(defn reverse (elems)
 (reduce_ elems (list) (fn _ (acc e) (cons e acc))))

(defn last (l) (car (reverse l)))

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

(defmacro cond (forms...)
 (if (empty forms)
  (quote (list))
  (let ((test (first forms))
        (arm  (second forms)))
      (list (quote if) test 
                       arm 
                       (list (quote apply) (quote cond) (list (quote quote) (rrest forms)))))))

(defn concat_ (elems)
 (reduce elems (fn _ (acc e) (reduce e acc (fn _ (acc_ e_) (push e_ acc_))))))

(defn concat (elems...)
 (concat_ elems))

(defn qq-body (form)
 (if (is-list form)
  (let ((f (first form)))
    (cond 
     (eq f (quote unq))  (list (second form))
     (eq f (quote unqs)) (second form)
     true (reduce form (list (list (quote list))) (fn _ (acc e)
                                                   (if (is-list e)
                                                    (push (qq-body e) acc)
                                                    (push (list e) acc))))))
  (list (list (quote quote) form))))

(defn noop (arg) arg)

(defmacro qq (form)
 (dbg! (concat_ (dbg! (qq-body form)))))