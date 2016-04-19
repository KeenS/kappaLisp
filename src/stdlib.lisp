(fset 'list (lambda (&rest args) args))

(fset 'assoc (lambda (key list)
               (if list
                (if (equalp key (car (car list)))
                    (car list)
                    (assoc key (cdr list)))
                nil)))
