(fset 'list (lambda (&rest args) args))
(fset 'defmacro (cons 'macro (lambda (name params &rest body)
                               (list 'fset (list 'quote name)
                                     (list 'cons (list 'quote 'macro) (cons 'lambda (cons params body)))))))
(defmacro defun (name params &rest body)
  (list 'fset (list 'quote name)
        (cons 'lambda (cons params body))))

(defmacro defparameter (name val)
  (list 'set (list 'quote name) val))

(defun assoc (key list)
  (if list
      (if (equalp key (car (car list)))
          (car list)
          (assoc key (cdr list)))
      nil))
