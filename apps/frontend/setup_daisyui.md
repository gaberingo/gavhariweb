## Install DaisyUI + Tailwindcss
```bash
# ../frontend/
curl -sL daisyui.com/fast | bash
```

## Config Trunk.toml
```toml
# ../frontend/Trunk.toml
[build]
target = "index.html"
dist = "dist"

[[hooks]]
stage = "build"
command = "sh"
command_arguments = ["-c", "./tailwindcss -i input.css -o $TRUNK_STAGING_DIR/output.css"]
```

## Index.html
```html
<!-- ../frontend/index.html -->
<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8" />
    <title>Yew App</title>
    <link rel="stylesheet" href="output.css" />
  </head>
  <body></body>
</html>
```