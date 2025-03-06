
; ;
; #_, ;
; (prn "hi")
; (comment (prn 123) (prn (keys (ns-map (symbol "clojure.core")))) (prn (vec (keys (cljx.core/all-ns-map)))) (prn (cljx.core/all-ns-map)) (prn (map ns-map (keys (cljx.core/all-ns-map)))) (prn (prn 1) (prn 2) (prn 3 (prn 4))) (prn {(symbol "clojure.core") (ns-map (symbol "clojure.core")) (symbol "cljx.core") (ns-map (symbol "cljx.core"))}))
;

;#_
(prn "hi")

;#_
(prn {
  (symbol "user/all-ns-map") (user/all-ns-map)
  (symbol "user/env-map") (user/env-map)
})

(prn {:namespaces (vec (keys (user/env-map)))})

#_
(prn (user/env-map))
