
set -e
set -u

PROJECT_NAME="${PWD##*/}"
CODECOV_TOKEN=${CODECOV_TOKEN:-}

OUTPUT_PROFILE="release"
if [ "$PROFILE" != "release" ]; then
  OUTPUT_PROFILE="debug"
fi
RUST_TARGET_FILE="target/${OUTPUT_PROFILE}/${PROJECT_NAME}"

echo
echo "**************************************"
echo "PROJECT_NAME: ${PROJECT_NAME}"
echo "PROFILE: ${PROFILE}"
echo "OUTPUT_PROFILE: ${OUTPUT_PROFILE}"
echo "RUST_TARGET_FILE: ${RUST_TARGET_FILE}"
echo "**************************************"
echo
