(local toml_edit (require :toml_edit))

(local config (toml_edit.parse ...))

(fn transform [exif]
  {
    "uri" (.. "file://" config.path (. exif 1))
    "width" (. exif 2)
    "height" (. exif 3)
    "mime" (. exif 4)
    "size" (. exif 5)
  })

(let [handle (assert (io.popen (.. "exiftool -p '$FileName|$ImageWidth|$ImageHeight|$MimeType|$FileSize' " config.path)))]
  (each [line (handle:lines)]
    (add_uri (transform (icollect [s (string.gmatch line "[^|]+")] s)))))


