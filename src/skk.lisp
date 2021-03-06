(defun skk-gadget-units-conversion (base v target)
  (* v (cdr (assoc target (cdr (assoc base skk-units-alist))))))

(defparameter skk-units-alist
  '(("mile" ("km" . 1.6093)
     ("yard" . 1760))

    ("yard" ("feet" . 3)
     ("cm" . 91.44))

    ("feet" ("inch" . 12)
     ("cm" . 30.48))

    ("inch" ("feet" . 0.5)
     ("cm" . 2.54))))

