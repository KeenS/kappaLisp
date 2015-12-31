(fset 'assoc (lambda (key list)
               (if list
                (let ((c (car list)))
                  (if (equal? key (car c))
                      (cdr c)
                      (assoc key (cdr list))))
                nil)))

(fset 'skk-gadget-units-conversion
      (lambda (base v target) (* v (cdr (assoc target (cdr (assoc base skk-units-alist)))))))

