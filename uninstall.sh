#!/bin/bash
echo "Uninstalling xclock..."
if [ -f "/usr/local/bin/xclock" ]; then
    sudo rm -f /usr/local/bin/xclock
    echo "xclock removed from /usr/local/bin"
else
    echo "xclock not found in /usr/local/bin"
fi
echo "Done."
