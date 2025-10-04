build-web:
  rm -rf ./docs
  trunk build --release --dist docs --minify --public-url ./
