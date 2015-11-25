(fset 'assoc (lambda (key list)
               (if list
                (let ((c (car list)))
                  (if (equal? key (car c))
                      (cdr c)
                      (assoc key (cdr list))))
                nil)))
