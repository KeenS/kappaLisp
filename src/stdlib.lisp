(fset 'assoc (lambda (key list)
               (if list
                (let ((c (car list)))
                  (if (equal? key (car c))
                      (cdr c)
                      (assoc key (cdr list))))
                nil)))

(fset 'skk-gadget-units-conversion
      (lambda (base v target) (* v (cdr (assoc target (cdr (assoc base skk-units-alist)))))))

(set 'skk-units-alist
     '(("mile" ("km" . 1.6093)
        ("yard" . 1760))

       ("yard" ("feet" . 3)
        ("cm" . 91.44))

       ("feet" ("inch" . 12)
        ("cm" . 30.48))

       ("inch" ("feet" . 0.5)
        ("cm" . 2.54))))
