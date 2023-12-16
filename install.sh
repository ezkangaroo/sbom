#!/bin/bash

# This file was initially generated by using:
# > https://github.com/kamilsk/godownloader
#
# but was subsequently modified.

parse_args() {
  BINDIR=${BINDIR:-/usr/local/bin}
  while getopts "b:h?x" arg; do
    case "$arg" in
      b) BINDIR="$OPTARG" ;;
      h | \?) usage "$0" ;;
      x) set -x ;;
    esac
  done
  shift $((OPTIND - 1))
  TAG=$1
}

execute() {
  tmpdir=$(mktmpdir)
  log_info "(download) downloading files into ${tmpdir}"
  http_download "${tmpdir}/${ARTIFACT_NAME}" "${ARTIFACT_URL}"

  srcdir="${tmpdir}"
  (cd "${tmpdir}" && untar "${ARTIFACT_NAME}")
  log_info "(installing binary) setting up bindir: $BINDIR"
  mkdir -p "$BINDIR" 2> /dev/null || sudo mkdir -p "$BINDIR"

  binexe="sbom"
  if [ "$OS" = "windows" ]; then
    binexe="${binexe}.exe"
  fi

  log_info "(installing binary) moving or copying from: $srcdir$binexe to $BINDIR/"
  mv "${srcdir}${binexe}" "${BINDIR}${binexe}" 2> /dev/null || sudo cp "${srcdir}${binexe}" "${BINDIR}/"
  log_info "(installed) ${BINDIR}/${binexe}"  
}

is_supported_platform() {
  platform=$1
  found=1
  case "$platform" in
    windows/amd64) found=0 ;;
    darwin/amd64) found=0 ;;
    darwin/arm64) found=0 ;;
    linux/amd64) found=0 ;;
    linux/arm64) found=0 ;;
  esac
  return $found
}
check_platform() {
  if is_supported_platform "$PLATFORM"; then
    true
  else
    log_info "(!!!!) platform $PLATFORM is not supported.  Make sure this script is up-to-date and file request at https://github.com/${GH_ID}/issues/new"
    exit 1
  fi
}
tag_to_version() {
  if [ -z "${TAG}" ]; then
    log_info "(download) checking GitHub for latest release"
  else
    log_info "(download) checking GitHub for release '${TAG}'"
  fi
  REALTAG=$(github_release "$OWNER/$REPO" "${TAG}") && true
  if test -z "$REALTAG"; then
    log_info "(!!!!) unable to find '${TAG}' - use 'latest' or see https://github.com/${GH_ID}/releases for details"
    exit 1
  fi
  TAG="$REALTAG"
  VERSION=${TAG#v}
}
adjust_format() {
  case ${ARCH} in
    windows) FORMAT=zip ;;
  esac
  true
}

cat /dev/null <<EOF
------------------------------------------------------------------------
https://github.com/client9/shlib - portable posix shell functions
Public domain - http://unlicense.org
https://github.com/client9/shlib/blob/master/LICENSE.md
but credit (and pull requests) appreciated.
------------------------------------------------------------------------
EOF
is_command() {
  command -v "$1" >/dev/null
}
echoerr() {
  echo "$@" 1>&2
}
log_info() {
  echoerr "$(log_prefix)" "$@"
}
uname_os() {
  os=$(uname -s | tr '[:upper:]' '[:lower:]')
  case "$os" in
    msys_nt) os="windows" ;;
  esac
  echo "$os"
}
uname_arch() {
  arch=$(uname -m)
  case $arch in
    x86_64) arch="amd64" ;;
    arm64) arch="arm64" ;;
  esac
  echo "${arch}"
}
untar() {
  tarball=$1
  case "${tarball}" in
    *.tar.gz | *.tgz) tar -xzf "${tarball}" ;;
    *.tar) tar -xf "${tarball}" ;;
    *.zip) unzip -o "${tarball}" ;;
    *)
      log_info "(!!!!) untar unknown archive format for ${tarball}"
      return 1
      ;;
  esac
}
mktmpdir() {
  test -z "$TMPDIR" && TMPDIR="$(mktemp -d)"
  mkdir -p "${TMPDIR}"
  echo "${TMPDIR}"
}
http_download_curl() {
  local_file=$1
  source_url=$2
  log_info "(download) querying via curl $2"
  header=$3
  if [ -n "$3" ]; then
    header="-H $header"
  fi
  # shellcheck disable=SC2086
  HTTP_CODE=$(curl -w '%{HTTP_CODE}' -sL $header -H "Cache-Control: no-cache" -o "$local_file" "$source_url") || (log_info "curl command failed." && return 1)
  return 0
}
http_download_wget() {
  local_file=$1
  source_url=$2
  log_info "(download) querying via wget $2"
  if [ -n "$3" ]; then
    header="--header $3"
  fi
  # shellcheck disable=SC2086
  HTTP_CODE=$(wget -q $header --no-cache --server-response -O "$local_file" "$source_url" 2>&1 | awk 'NR==1{print $2}') || (log_info "wget command failed." && return 1)
  return 0
}
http_download() {
  if is_command curl; then
    http_download_curl "$@"
  elif is_command wget; then
    http_download_wget "$@"
  else
    log_info "(!!!!) http_download unable to find wget or curl"
    return 1
  fi
  if [ -n "$HTTP_CODE" ] && [ "$HTTP_CODE" != 200 ] && [ "$HTTP_CODE" != 302 ]; then
    log_info "(!!!!) http_download received HTTP status $HTTP_CODE from $2"
    return 1
  fi
}
http_copy() {
  tmp=$(mktemp)
  if [ ! -w "$tmp" ];
  then
    log_info "(!!!!) Generated tempory file ${tmp} is not writable!"
  fi
  if [ ! -r "$tmp" ];
  then
    log_info "(!!!!) Generated tempory file ${tmp} is not readable!"
  fi

  http_download "${tmp}" "$1" "$2" || return 1
  body=$(cat "$tmp")
  rm -f "${tmp}"
  echo "$body"
}
github_release() {
  owner_repo=$1
  version=$2
  test -z "$version" && version="latest"
  giturl="https://github.com/${owner_repo}/releases/${version}"
  json=$(http_copy "$giturl" "Accept:application/json")
  test -z "$json" && return 1
  version=$(echo "$json" | tr -s '\n' ' ' | sed 's/.*"tag_name":"//' | sed 's/".*//')
  test -z "$version" && return 1
  echo "$version"
}

cat /dev/null <<EOF
------------------------------------------------------------------------
End of functions from https://github.com/client9/shlib
------------------------------------------------------------------------
EOF

get_binary_name() {
  case ${PLATFORM} in
    darwin/arm64)
      name=${APP_NAME}-universal-apple-darwin
      ;;
    darwin/amd64)
      name=${APP_NAME}-universal-apple-darwin
      ;;
    linux/arm64)
      name=${APP_NAME}-aarch64-unknown-linux-musl
      ;;
    linux/amd64)
      name=${APP_NAME}-x86_64-unknown-linux-musl
      ;;
    windows/amd64)
      name=${APP_NAME}-x86_64-pc-windows-gnu.zip
      ;;
  esac
  echo "$name"
}


APP_NAME="sbom"
OWNER=ezkangaroo
REPO="sbom"
FORMAT="tar.gz"
OS=$(uname_os)
ARCH=$(uname_arch)
GH_ID="$OWNER/$REPO"

log_prefix() {
	echo "$GH_ID"
}

PLATFORM="${OS}/${ARCH}"
GITHUB_DOWNLOAD=https://github.com/${OWNER}/${REPO}/releases/download

parse_args "$@"
check_platform
tag_to_version
adjust_format

NAME=$(get_binary_name)
ARTIFACT_NAME=${NAME}.${FORMAT}
ARTIFACT_URL=${GITHUB_DOWNLOAD}/${TAG}/${ARTIFACT_NAME}

log_info "(download) trying for sbom ${VERSION} binary for ${PLATFORM} at: ${ARTIFACT_URL}"
execute