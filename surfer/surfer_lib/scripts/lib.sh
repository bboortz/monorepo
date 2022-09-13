
set -e
set -u

PROJECT_NAME="${PWD##*/}"
CODECOV_TOKEN=${CODECOV_TOKEN:-}

OUTPUT_PROFILE="release"
if [ "$PROFILE" != "release" ]; then
  OUTPUT_PROFILE="debug"
fi
RUST_TARGET_FILE="target/${RUST_TARGET}/${OUTPUT_PROFILE}/lib${PROJECT_NAME}.rlib"

echo
echo "**************************************"
echo "PROJECT_NAME: ${PROJECT_NAME}"
echo "PROFILE: ${PROFILE}"
echo "RUST_TARGET: ${RUST_TARGET}"
echo "OUTPUT_PROFILE: ${OUTPUT_PROFILE}"
echo "RUST_TARGET_FILE: ${RUST_TARGET_FILE}"
echo "**************************************"
echo
