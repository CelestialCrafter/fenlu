(local lfs (require :lfs))
(let [schema "file://" path "/home/celestial/Documents/projects/fenlu/"]
  (each [file (lfs.dir path)] (add_uri (.. schema path file))))
