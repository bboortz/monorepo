
set -e
set -u

PROJECT_NAME="${PWD##*/}"
CODECOV_TOKEN=${CODECOV_TOKEN:-}

OUTPUT_PROFILE="release"
if [ "$PROFILE" != "release" ]; then
  OUTPUT_PROFILE="debug"
fi
RUST_TARGET_FILE="target/${OUTPUT_PROFILE}/${PROJECT_NAME}"
