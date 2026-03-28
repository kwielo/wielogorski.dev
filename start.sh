#!/bin/sh -e

# Prevent execution if this script was only partially downloaded
{

rc='\033[0m'
red='\033[0;31m'
cyan='\033[0;36m'

check() {
    exit_code=$1
    message=$2
    if [ "$exit_code" -ne 0 ]; then
        printf '%sERROR: %s%s\n' "$red" "$message" "$rc"
        exit 1
    fi
    unset exit_code
    unset message
}

REPO="kwielo/wielogorski.dev"
BINARY="wielo"

printf '%sKrzysztof Wielogórski — terminal portfolio%s\n' "$cyan" "$rc"

# ── Detect architecture ───────────────────────────────────────────────────────
findArch() {
    case "$(uname -m)" in
        x86_64|amd64)   arch="x86_64" ;;
        aarch64|arm64)  arch="aarch64" ;;
        *) check 1 "Unsupported architecture: $(uname -m)" ;;
    esac
}

# ── Detect OS ─────────────────────────────────────────────────────────────────
findOS() {
    case "$(uname -s)" in
        Linux)   os="unknown-linux-musl" ;;
        Darwin)  os="apple-darwin" ;;
        *) check 1 "Unsupported OS: $(uname -s)" ;;
    esac
}

# ── Build download URL ────────────────────────────────────────────────────────
getUrl() {
    echo "https://github.com/${REPO}/releases/latest/download/${BINARY}-${arch}-${os}"
}

findArch
findOS

temp_file=$(mktemp)
check $? "Creating temporary file"

printf 'Downloading %s (%s-%s)...\n' "$BINARY" "$arch" "$os"
curl -fsL "$(getUrl)" -o "$temp_file"
check $? "Downloading binary (check https://github.com/${REPO}/releases for available builds)"

chmod +x "$temp_file"
check $? "Making binary executable"

"$temp_file" "$@"
check $? "Running $BINARY"

rm -f "$temp_file"
check $? "Cleaning up"

} # End of wrapping
