set -ex

rustup target add wasm32-unknown-unknown

(cd website; trunk build)

VERSION="$(git rev-parse HEAD)"

cp website/dist/index.html "website/dist/index-${VERSION}.html"

ls -- website/dist/

gcloud storage cp \
  --recursive \
  --content-language=en \
  website/dist/ \
  gs://bookish-funicular/
