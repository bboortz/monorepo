
set -e
set -u

PROJECT_NAME="${PWD##*/}"
CODECOV_TOKEN=${CODECOV_TOKEN:-}

OUTPUT_PROFILE="release"
if [ "$PROFILE" != "release" ]; then
  OUTPUT_PROFILE="debug"
fi
