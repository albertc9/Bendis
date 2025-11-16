#!/bin/bash
# Bendis Environment Setup Script
# Add this line to your ~/.bashrc or ~/.zshrc:
#   source /path/to/bendis/settings.sh

# Get the directory where this script is located (compatible with bash and zsh)
if [ -n "${BASH_SOURCE[0]}" ]; then
    # bash
    BENDIS_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
elif [ -n "${(%):-%x}" ]; then
    # zsh
    BENDIS_ROOT="$(cd "$(dirname "${(%):-%x}")" && pwd)"
else
    # fallback
    BENDIS_ROOT="$(cd "$(dirname "$0")" && pwd)"
fi

# Add bendis binary to PATH
export PATH="${BENDIS_ROOT}/bendis/target/release:${PATH}"

# Add bender binary to PATH (from the bundled bender directory if it exists)
if [ -d "${BENDIS_ROOT}/bender/target/release" ]; then
    export PATH="${BENDIS_ROOT}/bender/target/release:${PATH}"
fi

# Also add cargo bin to PATH if it exists (for bender installed via cargo)
if [ -d "$HOME/.cargo/bin" ]; then
    export PATH="$HOME/.cargo/bin:${PATH}"
fi

# Print setup confirmation (only if BENDIS_VERBOSE is set or not in Powerlevel10k instant prompt)
# This prevents output during zsh initialization which breaks Powerlevel10k instant prompt
if [[ -n "${BENDIS_VERBOSE}" ]] || [[ "${TERM_PROGRAM}" == "vscode" ]] || [[ ! -v P9K_SSH ]]; then
    # Only show output if explicitly requested or not in a terminal with instant prompt
    if [[ -z "${ZSH_VERSION}" ]] || [[ -o interactive ]] && [[ -z "${POWERLEVEL9K_INSTANT_PROMPT}" ]]; then
        echo "✓ Bendis environment configured"
        echo "  BENDIS_ROOT: ${BENDIS_ROOT}"

        # Check if bendis is available
        if command -v bendis &> /dev/null; then
            echo "  ✓ bendis: $(which bendis)"
            echo "  ✓ version: $(bendis --version)"
        else
            echo "  ✗ bendis not found! Please build it first:"
            echo "    cd ${BENDIS_ROOT} && make build"
        fi

        # Check if bender is available
        if command -v bender &> /dev/null; then
            echo "  ✓ bender: $(which bender)"
        else
            echo "  ℹ bender not found in PATH (required for bendis to work)"
            echo "    Install from: https://github.com/pulp-platform/bender"
        fi

        echo ""
        echo "Usage: bendis init    # Initialize project"
        echo "       bendis update  # Update dependencies"
        echo "       bendis <cmd>   # Pass through to bender"
    fi
fi

# Define a helper function to show bendis status
bendis-status() {
    echo "✓ Bendis environment configured"
    echo "  BENDIS_ROOT: ${BENDIS_ROOT}"

    if command -v bendis &> /dev/null; then
        echo "  ✓ bendis: $(which bendis)"
        echo "  ✓ version: $(bendis --version)"
    else
        echo "  ✗ bendis not found! Please build it first:"
        echo "    cd ${BENDIS_ROOT} && make build"
    fi

    if command -v bender &> /dev/null; then
        echo "  ✓ bender: $(which bender)"
    else
        echo "  ℹ bender not found in PATH (required for bendis to work)"
        echo "    Install from: https://github.com/pulp-platform/bender"
    fi

    echo ""
    echo "Usage: bendis init    # Initialize project"
    echo "       bendis update  # Update dependencies"
    echo "       bendis <cmd>   # Pass through to bender"
}

# Note: Bendis is configured silently to avoid breaking Powerlevel10k instant prompt
# To see the configuration status, run: bendis-status
# Or set BENDIS_VERBOSE=1 before sourcing this script
